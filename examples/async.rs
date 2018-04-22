extern crate mio;
extern crate sensehat_stick;

use sensehat_stick::{JoyStick, JoystickIo};

use mio::{Events, Poll, PollOpt, Ready, Token};

fn main() {
    let poll = Poll::new().unwrap();
    let mut stick = JoystickIo::new(JoyStick::open().unwrap());
    let mut events = Events::with_capacity(64);
    poll.register(&stick, Token(0), Ready::readable(), PollOpt::edge())
        .unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            if event.token() == Token(0) && event.readiness().is_readable() {
                for ev in &stick.events().unwrap() {
                    println!("{:?}", ev);
                }
            }
        }
    }
}
