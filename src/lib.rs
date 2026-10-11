//! Geometric avatars from text. (Full docs come in Step 9.)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color;
mod error;
mod hash;
mod rng;

pub use color::{ ColorOptions, Palette, Rgba };
pub use error::Error;
pub use rng::{ Rng, random_seed };