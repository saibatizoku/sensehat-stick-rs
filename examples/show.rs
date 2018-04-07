// prints out the events received from the joystick
extern crate sensehat_stick;

use sensehat_stick::{JoyStick};

fn main() {
    let mut stick = JoyStick::open().unwrap();
    loop {
        for ev in &stick.events_no_sync().unwrap() {
            println!("{:?}", ev);
        }
    }
}
