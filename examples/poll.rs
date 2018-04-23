//! This example shows how to use `mio::Poll` with the
//! `sensehat_stick::JoyStick`, in a non-blocking fashion.
extern crate mio;
extern crate sensehat_stick;

use sensehat_stick::JoyStick;

use mio::{Events, Poll, PollOpt, Ready, Token};

const JOYSTICK: Token = Token(0);

fn main() {
    let mut stick = JoyStick::open().unwrap();
    let mut events = Events::with_capacity(64);

    let poll = Poll::new().unwrap();
    poll.register(&stick, JOYSTICK, Ready::readable(), PollOpt::edge())
        .unwrap();

    loop {
        // Internal handling of `epoll` event registration and retrieval.
        poll.poll(&mut events, None).unwrap();

        // This only lists the events that were retrieved by the last poll.
        for event in &events {
            if event.token() == JOYSTICK && event.readiness().is_readable() {
                match stick.events() {
                    Ok(evts) => for ev in &evts {
                        println!("{:?}", ev);
                    },
                    Err(e) => {
                        // Handling of spurious wake-ups, as usual.
                        if e.kind() == io::ErrorKind::WouldBlock {
                            continue;
                        }
                        bail!(e);
                    }
                }
            }
        }
    }
}
