pub fn greet() -> String {
    "Hello from crates.io!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello from crates.io!");
    }
}