#[derive(Debug)]
#[allow(dead_code)]
struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    fn new(value: i32) -> Result<Self, String> {
        if value < 1 {
            return Err("Invalid Value".to_owned());
        };
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess() -> Result<(), String> {
        let g = Guess::new(1).unwrap();
        if g.value == 1 {
            println!("DOne");
            Ok(())
        } else {
            Err(String::from("Invalid Value"))
        }
    }

    #[test]
    #[ignore]
    fn guess_another() -> Result<(), String> {
        let g = Guess::new(1).unwrap();
        if g.value == 1 {
            println!("DOne");
            Ok(())
        } else {
            Err(String::from("Invalid Value"))
        }
    }
}

// CHEAT_SHEET
// #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
// #[test] -> makes the function as test and runs when `cargo test`
// cargo test -- --show-output -> to print irrespective the test passes or fails
// #[should_panic(expected = "your_reason")] -> panics with your reason
// assert_eq!(left, right, "reason") -> checks equality
// assert_ne!(left, right. "reason") -> checks inequality
// assert!(value) -> checks trueness
// cargo test -- --test-threads=1 -> runs tests on one thread, in case of the inter-dependencies
// #[ignore] -> to ignore the test
// cargo test <test_name> -> will run the test if name matches
// cargo test -- --include-ignored -> include ignored tests
// cargo test --help -> help regarding the command
// cargo test -- --help -> help regarding what can be passed after "--" (--ignored -> to run the ignored tests)
