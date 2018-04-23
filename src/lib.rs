//! A Rust library for the Raspberry Pi Sense HAT Joystick.
//! =======================================================
//!
//! This library supports the joystick incorporated with the Sense HAT.
//!
//! The Joystick provides a driver for `evdev`.
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

use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;

// Device name provided by the hardware. We match against it.
const SENSE_HAT_EVDEV_NAME: &[u8; 31] = b"Raspberry Pi Sense HAT Joystick";

/// Direction in which the JoyStick is moved.
///
/// Internally, it matches the key-press events:
///
/// * `Direction::Enter = 28`
/// * `Direction::Up = 103`
/// * `Direction::Down = 108`
/// * `Direction::Left = 105`
/// * `Direction::Up = 106`
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
            c => bail!("unrecognized joystick code: {}", c),
        }
    }
}

/// The action that was executed with the given `Direction`.
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
            c => bail!("unrecognized joystick action: {}", c),
        }
    }
}

/// An event issued by the `JoyStick`. Provides a UNIX-timestamp in the form of
/// `std::time::Duration`, the `Direction`, and the `Action` that were issued by the `JoyStick`.
#[derive(Debug)]
pub struct JoyStickEvent {
    timestamp: Duration,
    direction: Direction,
    action: Action,
}

impl JoyStickEvent {
    fn new(timestamp: Duration, direction: Direction, action: Action) -> Self {
        JoyStickEvent {
            timestamp,
            direction,
            action,
        }
    }
}

/// A type representing the Sense HAT joystick device.
#[derive(Debug)]
pub struct JoyStick {
    device: Device,
}

impl JoyStick {
    /// Open the joystick device by name in the `/dev/input/event*` path on the filesystem.
    pub fn open() -> Result<Self, Error> {
        for entry in glob("/dev/input/event*")? {
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

    /// Returns a result with a `Vec<JoyStickEvent>`. This function will
    /// block the current thread until events are issued by the `JoyStick` device.
    pub fn events(&mut self) -> io::Result<Vec<JoyStickEvent>> {
        let events: Vec<JoyStickEvent> = self.device
            .events_no_sync()
            .map_err(|e| io::Error::from(e))?
            .filter(|ev| ev._type == 1)
            .map(|ev| {
                let secs = ev.time.tv_sec as u64;
                let nsecs = ev.time.tv_usec as u32 * 1_000;
                let time = Duration::new(secs, nsecs);

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
