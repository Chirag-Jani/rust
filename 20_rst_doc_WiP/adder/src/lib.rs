struct Rect {
    height: u32,
    width: u32,
}

#[allow(dead_code)]
impl Rect {
    fn is_valid(&self) -> bool {
        self.height != self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_rect() {
        let _rect = Rect {
            height: 32,
            width: 52,
        };

        assert!(rect.is_valid())
        // assert_ne!(2, 2, "Both values are same which should not be");
        // assert_eq!(2, 2, "Both values are not same which should be");
    }
}
