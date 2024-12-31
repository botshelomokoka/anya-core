pub mod alignment;
pub mod ml;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Initialize the anya-extensions library
pub fn init() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
