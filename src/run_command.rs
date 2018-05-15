use tokio::prelude::{Async, Future, Poll};
use tokio::run;
use tokio::timer::Delay;

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use super::{AsyncChild, Error, Output};

#[inline]
pub fn run_command(mut command: Command, timeout: f32) -> Result<Output, Error> {
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let child_future = ChildFuture::new(AsyncChild::new(child), timeout);

    let (sender, receiver) = channel();

    run(child_future.then(move |result| {
        let _ = sender.send(result);
        Ok(())
    }));

    receiver.recv().unwrap()
}
struct ChildFuture {
    child: AsyncChild,
    output: Option<Output>,
    delay: Delay,
}

impl ChildFuture {
    #[inline]
    fn new(child: AsyncChild, timeout: f32) -> Self {
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
        match self.child.poll_exit() {
            Ok(Async::Ready(status)) => {
                {
                    let stdout = self.child.inner.stdout.as_mut();
                    let output = self.output.as_mut();
                    stdout.map(|stdout| {
                        output.map(|output| stdout.read_to_string(&mut output.stdout))
                    });
                }
                {
                    let stderr = self.child.inner.stderr.as_mut();
                    let output = self.output.as_mut();
                    stderr.map(|stderr| {
                        output.map(|output| stderr.read_to_string(&mut output.stderr))
                    });
                }

                let mut output = self.output.take().unwrap();

                output.error = if status.success() {
                    None
                } else {
                    Some((&status).into())
                };

                return Ok(Async::Ready(output));
            }
            Ok(Async::NotReady) => (),
            Err(e) => return Err(e.into()),
        }

        match self.delay.poll() {
            Ok(Async::Ready(_)) => {
                let _ = self.child.kill();
                return Err(Error::Timeout);
            }
            _ => return Ok(Async::NotReady),
        }
    }
}
