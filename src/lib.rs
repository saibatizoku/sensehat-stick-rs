#[cfg(feature = "linux-evdev")]
extern crate evdev;
#[macro_use]
extern crate failure;
extern crate glob;

use failure::Error;
use glob::glob;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
