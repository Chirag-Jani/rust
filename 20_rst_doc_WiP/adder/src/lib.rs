#[derive(Debug)]
struct Guess {
    value: i32,
}

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
    #[should_panic]
    fn guess() {
        let g = Guess::new(23).unwrap();
        assert_eq!(g.value, 23, "value mismatched")
    }
}
