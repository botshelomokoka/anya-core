#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_networking_init() {
        assert!(init().is_ok());
    }
}