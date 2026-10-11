//! The one error type for the whole crate.

use std::borrow::Cow;
use std::fmt;

/// Everything that can go wrong when building an avatar.
///
/// `#[non_exhaustive]` means we can add new variants in a later version
/// without breaking code that `match`es on this enum.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The requested size is outside the allowed range.
    InvalidSize {
        /// The size that was asked for, in pixels.
        size: u32,
        /// Smallest allowed size.
        min: u32,
        /// Largest allowed size.
        max: u32,
    },
    /// Padding must be at least 0.0 and less than 0.5.
    InvalidPadding(f32),
    /// A color setting is out of range. The text says which one and why.
    ///
    /// It's a [`Cow`], so fixed messages cost nothing and messages that
    /// include the bad value can be built with `format!`.
    InvalidColor(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSize { size, min, max } => {
                write!(
                    f,
                    "size {size} is out of range (allowed: {min} to {max} pixels)"
                )
            }
            Error::InvalidPadding(padding) => {
                write!(
                    f,
                    "padding {padding} is out of range (allowed: 0.0 up to, not including, 0.5)"
                )
            }
            Error::InvalidColor(reason) => write!(f, "invalid color setting: {reason}"),
        }
    }
}

// This one line lets our Error work with `?` in functions that return
// `Result<_, Box<dyn std::error::Error>>`, and with crates like `anyhow`.
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_are_readable() {
        let err = Error::InvalidSize {
            size: 4,
            min: 16,
            max: 4096,
        };
        assert_eq!(
            err.to_string(),
            "size 4 is out of range (allowed: 16 to 4096 pixels)"
        );
    }

    #[test]
    fn color_messages_can_be_fixed_or_formatted() {
        // A fixed message: no allocation.
        let fixed = Error::InvalidColor("bad hue".into());
        assert_eq!(fixed.to_string(), "invalid color setting: bad hue");

        // A message that includes the bad value.
        let formatted = Error::InvalidColor(format!("saturation {} is too high", 1.5).into());
        assert_eq!(
            formatted.to_string(),
            "invalid color setting: saturation 1.5 is too high"
        );
    }
}
