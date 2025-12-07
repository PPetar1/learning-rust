pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
            "Guess value must be between 1 and 100, got {}.",
            value
            );
        }

        Guess { value }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);// assert_eq!() and assert_ne!() macros check if the two values are
                              // equal or not equal respectively, otherwise they panic; values that
                              // are passed to them need to implement PartialEq and Debug traits
                              // which can be derived using #[derive(PartialEq, Debug)]
    }

    #[test]
    fn test_that_fails() {
        //panic!("Forcing the test to fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));// assert!(bool) macro panics if we pass a false value
                                           // to it and is useful in tests
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger), 
            "A custom message in case of an error");// You can also add a custom message to the
                                                    // assert that will be passed to the format!
                                                    // macro (so you can use {}) and then to
                                                    // panic!() in case of an error
    }

    // should_panic attribute

    #[test]
    #[should_panic] // We can make #[should_panic] more precise by using #[should_panic(expected =
                    // "This string must be contained in the error message")] which will count the
                    // test as passed only if the panic is made with the message that contains the
                    // text we passed above. This way limits the possibility for a false negative
                    // where some other part of the code panics instead of the one we were
                    // expecting
    fn greater_than_100() {// Here we want to test error handling so this code should panic. For
                           // that we use the should_panic attribute like so and this test passes
                           // if the code inside panics
        Guess::new(200);
    }

    // Tests that use Result<T, E>

    #[test]
    // When using Result we cannot use #[should_panic]
    #[ignore] // ignore tells cargo not to run this test unless we specifically request it with
              // cargo test -- --ignored or ~||~ --include_ignored
    fn it_works() -> Result<(), String> {
        if add(2,2) == 4 {
            Ok(())
        } else {
            Err(String::from("Test failed, 2 + 2 != 4"))
        }
    }
}
// cargo test can take command line arguments to change some behaviours, some go to cargo test and
// some to the resulting test binary. The two are separated by -- so its cargo test
// arguments_to_cargo -- arguments_to_bin You can see arguments for cargo test with cargo test
// --help and for the binary with cargo test -- --help
