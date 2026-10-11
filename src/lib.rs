//! # ferrav
//!
//! Geometric avatars for any text, as SVG or PNG, in pure Rust.
//!
//! ```
//! use ferrav::Avatar;
//!
//! // Same text, same avatar, every time.
//! let svg = Avatar::new("sagar").to_svg();
//! assert_eq!(svg, Avatar::new("sagar").to_svg());
//!
//! // Same text, a different avatar on every call.
//! let avatar = Avatar::random("sagar");
//!
//! // Keep the seed to reproduce the same avatar with the same settings.
//! let again = Avatar::builder()
//!     .seed(avatar.seed())
//!     .build()?;
//!
//! assert_eq!(avatar, again);
//!
//! # Ok::<(), ferrav::Error>(())
//! ```
//!
//! Change any setting with the builder:
//!
//! ```
//! use ferrav::{Avatar, ColorOptions, Mode, Pixel, Rgba};
//!
//! let avatar = Avatar::builder()
//!     .text("sagar")
//!     .mode(Mode::Random)
//!     .size(256)
//!     .padding(0.1)
//!     .background(Rgba::WHITE)
//!     .colors(ColorOptions {
//!         hues: vec![200.0],
//!         ..Default::default()
//!     })
//!     .style(Pixel::default())
//!     .build()?;
//!
//! # Ok::<(), ferrav::Error>(())
//! ```
//!
//! ## How it works
//!
//! ```text
//! text
//!  |
//!  v
//! seed
//!  |
//!  v
//! style + palette
//!  |
//!  v
//! scene
//!  |
//!  +----> SVG
//!  |
//!  +----> RGBA pixels
//!  |
//!  +----> PNG
//! ```
//!
//! A seed controls every random choice, making avatars reproducible when the
//! same settings are used.
//!
//! ## Cargo features
//!
//! - `png` (enabled by default): PNG output through [`Avatar::to_png`].
//!
//! Disable default features for a dependency-free SVG/RGBA build:
//!
//! ```toml
//! ferrav = { version = "0.1", default-features = false }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// On docs.rs only: label items that need a cargo feature, like `to_png`.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod avatar;
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

pub use avatar::{Avatar, AvatarBuilder, MAX_SIZE, MIN_SIZE, Mode};
pub use color::{ColorOptions, Palette, Rgba};
pub use error::Error;
pub use geometry::{Cell, Point, Rotation};
#[cfg(feature = "png")]
pub use png::encode_png;
pub use raster::render_rgba;
pub use rng::{Rng, random_seed};
pub use scene::{Geometry, Scene, Shape};
pub use style::{Geometric, Pixel, Style};
pub use svg::render_svg;