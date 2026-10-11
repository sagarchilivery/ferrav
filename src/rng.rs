//! Random numbers.
//!
//! A *seed* is a single `u64`. [`Rng`] turns a seed into an endless stream of
//! numbers that look random but are fully repeatable: same seed, same stream.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// A small, fast pseudo-random number generator (the SplitMix64 algorithm).
///
/// Not for cryptography. It's for picking shapes and colors.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Creates a generator.
    ///
    /// The same seed always produces the same numbers.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Returns the next random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        // Step the state forward, then scramble it so the bits look random.
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);

        z ^ (z >> 31)
    }

    /// Returns a whole number from `0` up to (not including) `n`.
    ///
    /// Uses rejection sampling to avoid modulo bias.
    ///
    /// # Panics
    ///
    /// Panics if `n` is 0.
    pub fn below(&mut self, n: usize) -> usize {
        assert!(n > 0, "Rng::below needs n > 0");

        let n = n as u64;
        let limit = u64::MAX - (u64::MAX % n);

        loop {
            let value = self.next_u64();

            if value < limit {
                return (value % n) as usize;
            }
        }
    }

    /// Returns a decimal number from `0.0` up to (not including) `1.0`.
    pub fn next_f32(&mut self) -> f32 {
        // Keep the top 24 bits: exactly as many as an f32 can represent.
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Returns `true` with the given probability.
    ///
    /// `0.0` (or less) = never, `1.0` (or more) = always. `NaN` counts as never.
    pub fn chance(&mut self, probability: f32) -> bool {
        self.next_f32() < probability
    }

    /// Picks one item from a slice.
    ///
    /// # Panics
    ///
    /// Panics if the slice is empty.
    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    /// Creates a new independent `Rng` from this generator.
    ///
    /// Useful when different parts of generation need their own random stream.
    pub fn fork(&mut self) -> Self {
        Self::new(self.next_u64())
    }
}

/// Returns a fresh, unpredictable seed.
///
/// Rust's `HashMap` already asks the operating system for random keys, and
/// every new `RandomState` gets different ones. Hashing the current time with
/// one gives a new random number on every call, without adding a dependency.
pub fn random_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    let mut hasher = RandomState::new().build_hasher();

    hasher.write_u128(nanos);

    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_numbers() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);

        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn numbers_stay_in_range() {
        let mut rng = Rng::new(7);

        for _ in 0..1000 {
            assert!(rng.below(5) < 5);

            let f = rng.next_f32();
            assert!((0.0..1.0).contains(&f));
        }
    }

    #[test]
    fn chance_respects_probability() {
        let mut rng = Rng::new(1);

        for _ in 0..100 {
            assert!(rng.chance(1.0));
            assert!(!rng.chance(0.0));
        }
    }

    #[test]
    fn out_of_range_probabilities_never_panic() {
        let mut rng = Rng::new(1);

        for _ in 0..100 {
            assert!(rng.chance(1.5));
            assert!(!rng.chance(-0.5));
            assert!(!rng.chance(f32::NAN));
        }
    }

    #[test]
    fn fork_is_deterministic() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);

        assert_eq!(a.fork().next_u64(), b.fork().next_u64());
    }

    #[test]
    fn random_seeds_differ() {
        assert_ne!(random_seed(), random_seed());
    }
}