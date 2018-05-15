extern crate libc;
extern crate tokio_signal;

use std::io;
use std::os::unix::prelude::*;
use std::process::{Child, ExitStatus};

use self::tokio_signal::unix::Signal;
use self::tokio_signal::IoFuture;
use tokio::prelude::future::FlattenStream;
use tokio::prelude::{Async, Future, Poll, Stream};

pub struct AsyncChild {
    pub inner: Child,
    reaped: bool,
    sigchld: FlattenStream<IoFuture<Signal>>,
}

impl AsyncChild {
    #[inline]
    pub fn new(inner: Child) -> Self {
        AsyncChild {
            inner: inner,
            reaped: false,
            sigchld: Signal::new(libc::SIGCHLD).flatten_stream(),
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.inner.id()
    }

    #[inline]
    pub fn kill(&mut self) -> io::Result<()> {
        if !self.reaped {
            self.inner.kill()?;
            let _ = self.try_wait(true);
        }

        Ok(())
    }

    #[inline]
    pub fn poll_exit(&mut self) -> Poll<ExitStatus, io::Error> {
        loop {
            if let Some(e) = try!(self.try_wait(false)) {
                return Ok(e.into());
            }
            if try!(self.sigchld.poll()).is_not_ready() {
                return Ok(Async::NotReady);
            }
        }
    }

    #[inline]
    fn try_wait(&mut self, block_on_wait: bool) -> io::Result<Option<ExitStatus>> {
        assert!(!self.reaped);
        let exit = try!(try_wait_process(self.id() as libc::pid_t, block_on_wait));

        if let Some(_) = exit {
            self.reaped = true;
        }

        Ok(exit)
    }
}

#[inline]
fn try_wait_process(id: libc::pid_t, block_on_wait: bool) -> io::Result<Option<ExitStatus>> {
    let wait_flags = if block_on_wait { 0 } else { libc::WNOHANG };
    let mut status = 0;

    loop {
        match unsafe { libc::waitpid(id, &mut status, wait_flags) } {
            0 => return Ok(None),
            n if n < 0 => {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(err);
            }
            n => {
                assert_eq!(n, id);
                return Ok(Some(ExitStatus::from_raw(status)));
            }
        }
    }
}
