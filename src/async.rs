//! Support for asynchronous joystick I/O.
use super::JoyStick;

use mio::{Poll, PollOpt, Ready, Token};
use mio::event::Evented;
use mio::unix::EventedFd;

use std::io;

/// Event-pollable wrapper for the JoyStick's `RawFd`.
pub struct JoystickIo {
    stick: JoyStick,
}

impl JoystickIo {
    pub fn new(stick: JoyStick) -> Self {
        JoystickIo { stick }
    }
}

impl Evented for JoystickIo {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.stick.fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.stick.fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.stick.fd()).deregister(poll)
    }
}
