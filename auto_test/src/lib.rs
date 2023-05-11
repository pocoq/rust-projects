pub mod mod_write_test {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
        // String::from("Hello!")
    }

    pub struct Guess {
        pub value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }

            Guess { value }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_write_test as m1;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = m1::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = m1::Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = m1::greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
	#[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        m1::Guess::new(200);
    }

	#[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
