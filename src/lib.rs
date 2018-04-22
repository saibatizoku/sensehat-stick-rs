#[cfg(feature = "linux-evdev")]
extern crate evdev;
#[macro_use]
extern crate failure;
extern crate glob;
#[cfg(feature = "async")]
extern crate mio;

#[cfg(feature = "async")]
mod async;

use evdev::Device;
use failure::Error;
use glob::glob;

use std::os::unix::io::RawFd;

#[cfg(feature = "async")]
pub use async::JoystickIo;

const SENSE_HAT_EVDEV_NAME: &[u8; 31] = b"Raspberry Pi Sense HAT Joystick";

#[derive(Debug)]
pub enum Direction {
    Enter = 28,
    Up = 103,
    Down = 108,
    Left = 105,
    Right = 106,
}

impl Direction {
    fn try_from(code: usize) -> Result<Self, Error> {
        match code {
            c if c == Direction::Enter as usize => Ok(Direction::Enter),
            c if c == Direction::Up as usize => Ok(Direction::Up),
            c if c == Direction::Down as usize => Ok(Direction::Down),
            c if c == Direction::Left as usize => Ok(Direction::Left),
            c if c == Direction::Right as usize => Ok(Direction::Right),
            _ => bail!("unrecognized joystick code"),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Release = 0,
    Press = 1,
    Hold = 2,
}

impl Action {
    fn try_from(code: usize) -> Result<Self, Error> {
        match code {
            c if c == Action::Press as usize => Ok(Action::Press),
            c if c == Action::Release as usize => Ok(Action::Release),
            c if c == Action::Hold as usize => Ok(Action::Hold),
            _ => bail!("unrecognized joystick action"),
        }
    }
}

#[derive(Debug)]
pub struct JoyStickEvent {
    timestamp: usize,
    direction: Direction,
    action: Action,
}

impl JoyStickEvent {
    fn new(timestamp: usize, direction: Direction, action: Action) -> Self {
        JoyStickEvent {
            timestamp,
            direction,
            action,
        }
    }
}

#[derive(Debug)]
pub struct JoyStick {
    device: Device,
}

impl JoyStick {
    pub fn open() -> Result<Self, Error> {
        for entry in glob("/sys/class/input/event*")? {
            match entry {
                Ok(path) => {
                    let device = Device::open(&path)?;
                    if device.name().as_bytes() == SENSE_HAT_EVDEV_NAME {
                        return Ok(JoyStick { device });
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        bail!("No Joystick found")
    }

    pub fn events_no_sync(&mut self) -> Result<Vec<JoyStickEvent>, Error> {
        let events: Vec<JoyStickEvent> = self.device
            .events_no_sync()?
            .map(|ev| {
                let time = ev.time.tv_sec as usize;
                let direction = Direction::try_from(ev.code as usize).unwrap();
                let action = Action::try_from(ev.value as usize).unwrap();
                JoyStickEvent::new(time, direction, action)
            })
            .collect();
        Ok(events)
    }

    /// Returns the raw file-descriptor, `RawFd`, for the the Joystick.
    pub fn fd(&self) -> RawFd {
        self.device.fd()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
