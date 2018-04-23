//! Prints out the events received from the joystick, in a
//! blocking fashion.
extern crate sensehat_stick;

use sensehat_stick::JoyStick;

fn main() {
    let mut stick = JoyStick::open().unwrap();
    loop {
        // This call will block the current thread until
        // an event is triggered on the joystick.
        for ev in &stick.events().unwrap() {
            println!("{:?}", ev);
        }
    }
}
