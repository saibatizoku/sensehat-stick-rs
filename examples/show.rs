// prints out the events received from the joystick
extern crate sensehat_stick;

use sensehat_stick::JoyStick;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut stick = JoyStick::open().unwrap();
    loop {
        for ev in &stick.events().unwrap() {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => println!("{}", n.as_secs()),
                Err(_) => println!("SystemTime before UNIX EPOCH"),
            }
            println!("{:?}", ev);
        }
    }
}
