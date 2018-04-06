#[cfg(feature = "linux-evdev")]
extern crate evdev;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
