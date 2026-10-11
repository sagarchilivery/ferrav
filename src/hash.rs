//! Converts text into a deterministic number.
//!
//! The same input always produces the same output.

/// FNV-1a 64-bit hash function.
///
/// This is not cryptographically secure and should not be used for passwords
/// or security-sensitive purposes. It is suitable for generating deterministic
/// values such as avatar seeds.
pub fn fnv1a(text: &str) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET_BASIS;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_published_test_values() {
        assert_eq!(fnv1a(""), 0xcbf29ce484222325);
        assert_eq!(fnv1a("a"), 0xaf63dc4c8601ec8c);
    }

    #[test]
    fn same_text_same_number() {
        assert_eq!(fnv1a("sagar"), fnv1a("sagar"));
        assert_ne!(fnv1a("sagar"), fnv1a("Sagar"));
    }
}
