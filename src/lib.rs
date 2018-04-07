#[cfg(feature = "linux-evdev")]
extern crate evdev;
#[macro_use]
extern crate failure;

use failure::Error;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
