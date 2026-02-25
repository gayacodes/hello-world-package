pub fn greet() -> String {
    "Hello from AWS CodeArtifact!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello from AWS CodeArtifact!");
    }
}