use std::fmt;

#[derive(Copy, Clone)]
pub struct LargeInteger{
    low: u64,
    high: u64,
}

impl LargeInteger{
    pub fn default() -> LargeInteger{
        LargeInteger{
            low: 0,
            high: 0,
        }
    }

    pub fn new(low: u64, high: u64) -> LargeInteger{
        LargeInteger{
            low: low,
            high: high,
        }
    }
}

impl PartialEq for LargeInteger {
    fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }
}

impl Eq for LargeInteger {}

impl fmt::Debug for LargeInteger
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LargeInteger")
         .field("low", &self.low)
         .field("high", &self.high)
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_integer_with_values() {
        let large_integer = LargeInteger::new(123, 456);
        assert_eq!(large_integer.low, 123);
        assert_eq!(large_integer.high, 456);
    }

    #[test]
    fn test_large_integer_with_default_values() {
        let large_integer = LargeInteger::default();
        assert_eq!(large_integer.low, 0);
        assert_eq!(large_integer.high, 0);
    }

    #[test]
    fn test_large_integer_equality() {
        let large_integer1 = LargeInteger::new(123, 456);
        let large_integer2 = LargeInteger::new(123, 456);
        assert_eq!(large_integer1, large_integer2);
    }

    #[test]
    fn test_large_integer_inequality() {
        let large_integer1 = LargeInteger::new(123, 456);
        let large_integer2 = LargeInteger::new(789, 101112);
        assert_ne!(large_integer1, large_integer2);
    }
}
