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
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            if event.token() == JOYSTICK && event.readiness().is_readable() {
                match stick.events() {
                    Ok(evts) => {
                        for ev in &evts {
                            println!("{:?}", ev);
                        }
                    }
                    Err(e) => {
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
