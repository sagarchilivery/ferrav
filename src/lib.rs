//! Geometric avatars from text. (Full docs come in Step 9.)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color;
mod error;
mod geometry;
mod hash;
mod rng;
mod scene;
mod style;

pub use color::{ ColorOptions, Palette, Rgba };
pub use error::Error;
pub use geometry::{ Point, Cell, Rotation };
pub use rng::{ Rng, random_seed };
pub use scene::{ Geometry, Shape, Scene };
pub use style::{ Geometric, Style };