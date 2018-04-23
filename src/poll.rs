//! Support for efficient polling for the joystick file-descriptor.
use super::JoyStick;

use mio::{Poll, PollOpt, Ready, Token};
use mio::event::Evented;
use mio::unix::EventedFd;

use std::io;

impl Evented for JoyStick {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd()).deregister(poll)
    }
}
