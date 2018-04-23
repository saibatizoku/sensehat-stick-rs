A Rust library for the Raspberry Pi Sense HAT Joystick.
=======================================================

[![crates.io](https://img.shields.io/crates/v/sensehat-stick.svg)](https://crates.io/crates/sensehat-stick)
[![docs](https://docs.rs/sensehat-stick/badge.svg)](https://docs.rs/sensehat-stick)


The joystick integrated on the [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/), is well integrated to Linux systems. The Sense HAT hardware provides a driver for `evdev`, or `event device` interface in the Linux kernel.

As such, the `evdev` file-descriptor for the joystick, can be read for events issued by someone pushing the joystick.

This library provides a thread-safe, strong-typed, high-level API for the joystick, treating it as you would any other input device using `evdev`.

# Usage

To use this crate with the default features, add this to your `Cargo.toml`:
```cargo
[dependencies]
sensehat-stick = "0.1"
```

or, to manually specify the features::

```cargo
[dependencies]
sensehat-stick = { version = "0.1", default-features = false, features = ["poll"] }
```

# Features

`default`
---------
By default, the `linux-evdev`, and `poll` features are included.

`linux-evdev`
-------------
In `default`. Makes use of the `evdev` interface.

`poll`
------
In `default`. Provides efficient-polling capabilities by implementing `mio::Evented` for `JoyStick`.

# Example
```rust
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
```

For more, read the indiviual [examples](./examples/).
