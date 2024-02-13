pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_name() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        let result = add(3, 4);
        panic!("Make this test fail");
    }
}
