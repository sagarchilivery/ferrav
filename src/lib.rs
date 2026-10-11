//! Geometric avatars from text. (Full docs come in Step 9.)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color;
mod error;
mod geometry;
mod hash;
#[cfg(feature = "png")]
mod png;
mod raster;
mod rng;
mod scene;
mod style;
mod svg;

pub use color::{ ColorOptions, Palette, Rgba };
pub use error::Error;
pub use geometry::{ Point, Cell, Rotation };
#[cfg(feature = "png")]
pub use png::encode_png;
pub use raster::render_rgba;
pub use rng::{ Rng, random_seed };
pub use scene::{ Geometry, Shape, Scene };
pub use style::{ Geometric, Pixel, Style };
pub use svg::render_svg;