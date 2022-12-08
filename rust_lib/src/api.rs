pub fn increase(current: usize) -> usize {
    current + 1
}

pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}

#[cfg(test)]
mod tests {
    use crate::api::{greet, increase};

    #[test]
    fn it_works() {
        assert_eq!(greet(), "Hello from Rust! 🦀");
    }
    #[test]
    fn test_increase() {
        let counter: usize = 1;
        assert_eq!(increase(counter), 2);
    }
}
