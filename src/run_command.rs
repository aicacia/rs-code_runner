use tokio::prelude::task::current;
use tokio::prelude::{Async, Future, Poll};
use tokio::run;
use tokio::timer::Delay;

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use super::{Error, Output};

#[inline]
pub fn run_command(mut command: Command, timeout: f32) -> Result<Output, Error> {
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let child_future = ChildFuture::new(child, timeout);

    let (sender, receiver) = channel();

    run(child_future.then(move |result| {
        let _ = sender.send(result);
        Ok(())
    }));

    receiver.recv().unwrap()
}

struct ChildFuture {
    child: Child,
    output: Option<Output>,
    delay: Delay,
}

impl ChildFuture {
    #[inline]
    fn new(child: Child, timeout: f32) -> Self {
        ChildFuture {
            child: child,
            output: Some(Output::default()),
            delay: Delay::new(Instant::now() + Duration::from_millis((timeout * 1000.0) as u64)),
        }
    }
}

impl Future for ChildFuture {
    type Item = Output;
    type Error = Error;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        {
            let stdout = self.child.stdout.as_mut();
            let output = self.output.as_mut();
            stdout.map(|stdout| output.map(|output| stdout.read_to_string(&mut output.stdout)));
        }
        {
            let stderr = self.child.stderr.as_mut();
            let output = self.output.as_mut();
            stderr.map(|stderr| output.map(|output| stderr.read_to_string(&mut output.stderr)));
        }

        match self.child.try_wait() {
            Ok(Some(status)) => {
                let mut output = self.output.take().unwrap();

                output.error = if status.success() {
                    None
                } else {
                    Some((&status).into())
                };

                return Ok(Async::Ready(output));
            }
            Ok(None) => {
                current().notify();
            }
            Err(e) => return Err(e.into()),
        }

        match self.delay.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            _ => {
                let _ = self.child.kill();
                Err(Error::Timeout)
            }
        }
    }
}
