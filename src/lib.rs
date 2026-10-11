//! # ferrav
//!
//! **Turn any text into a unique little picture.** Give ferrav a username, an
//! email or an ID, and it draws an avatar for it, as an SVG or a PNG.
//! Same text, same picture, every time.
//!
//! <p>
//!   <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/sagar.png" width="72" alt="avatar for sagar">
//!   <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/ferrav.png" width="72" alt="avatar for ferrav">
//!   <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/hello.png" width="72" alt="avatar for hello@example.com">
//!   <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/style-pixel.png" width="72" alt="pixel-style avatar">
//!   <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/color-pastel.png" width="72" alt="pastel avatar">
//! </p>
//!
//! For a longer guide with more pictures, see the
//! [README](https://github.com/sagarchilivery/ferrav#readme). For complete
//! programs you can run, see the
//! [examples folder](https://github.com/sagarchilivery/ferrav/tree/main/examples).
//!
//! # Quick start
//!
//! Add ferrav to your project:
//!
//! ```bash
//! cargo add ferrav
//! ```
//!
//! Then make your first avatar. Put this inside `fn main()` in `src/main.rs`:
//!
//! ```no_run
//! use ferrav::Avatar;
//!
//! let avatar = Avatar::new("sagar");
//!
//! std::fs::write("sagar.svg", avatar.to_svg()).unwrap();
//! # #[cfg(feature = "png")]
//! std::fs::write("sagar.png", avatar.to_png()).unwrap();
//! ```
//!
//! Run it with `cargo run` and open `sagar.svg` or `sagar.png`.
//!
//! **SVG or PNG?** SVG is text that browsers draw directly: tiny, and sharp at
//! any size, so it's best for websites. PNG is a normal image file that works
//! everywhere else. You can also get raw pixels with [`Avatar::to_rgba`].
//!
//! # Same avatar, or a random one?
//!
//! | | Code | Same text gives... |
//! | --- | --- | --- |
//! | **Deterministic** (default) | [`Avatar::new`] | the same avatar, every time, on every computer |
//! | **Random** | [`Avatar::random`] | a new avatar on every call |
//!
//! Every avatar has a **seed**: one number that describes it completely.
//! Save it, and you can get the exact same avatar back later:
//!
//! ```
//! use ferrav::Avatar;
//!
//! let avatar = Avatar::random("sagar");
//! let seed = avatar.seed(); // a u64: save it, e.g. in your users table
//!
//! // Later, even on another day or another server:
//! let same = Avatar::from_seed(seed);
//! assert_eq!(avatar, same);
//! ```
//!
//! # Change any setting
//!
//! Start with [`Avatar::builder`], set only what you want, and finish with
//! [`.build()`](AvatarBuilder::build). Everything you don't set keeps a
//! sensible default.
//!
//! ```
//! use ferrav::{Avatar, ColorOptions, Pixel, Rgba};
//!
//! let avatar = Avatar::builder()
//!     .text("sagar")
//!     .size(256)                 // width and height in pixels
//!     .padding(0.1)              // 10% empty space on each side
//!     .background(Rgba::WHITE)   // default: transparent
//!     .colors(ColorOptions {
//!         hues: vec![210.0],     // only blues
//!         ..Default::default()   // keep the defaults for everything else
//!     })
//!     .style(Pixel::default())   // GitHub-style squares
//!     .build()?;
//! # Ok::<(), ferrav::Error>(())
//! ```
//!
//! | Setting | Default | Allowed values |
//! | --- | --- | --- |
//! | [`text`](AvatarBuilder::text) | `""` | any text |
//! | [`mode`](AvatarBuilder::mode) | [`Mode::Deterministic`] | [`Mode::Deterministic`], [`Mode::Random`] |
//! | [`seed`](AvatarBuilder::seed) | none | any `u64` (replaces text and mode) |
//! | [`size`](AvatarBuilder::size) | `128` | [`MIN_SIZE`] (16) to [`MAX_SIZE`] (4096) pixels |
//! | [`padding`](AvatarBuilder::padding) | `0.08` | `0.0` up to (not including) `0.5` |
//! | [`background`](AvatarBuilder::background) | transparent | any [`Rgba`] color |
//! | [`colors`](AvatarBuilder::colors) | [`ColorOptions::default()`] | see [`ColorOptions`] |
//! | [`style`](AvatarBuilder::style) | [`Geometric`] | [`Geometric`], [`Pixel`], or your own [`Style`] |
//!
//! ## Styles
//!
//! - [`Geometric`] (the default): symmetric shapes on a 4×4 grid.
//! - [`Pixel`]: GitHub-style squares, mirrored left to right. Its `grid`
//!   setting is the number of squares per row, and `density` is the chance
//!   that each square is filled.
//!
//! ## Colors
//!
//! [`ColorOptions`] uses three simple ideas:
//!
//! - **Hue** is *which* color, as an angle on the color wheel: `0` red,
//!   `60` yellow, `120` green, `180` cyan, `240` blue, `300` magenta.
//! - **Saturation** is *how colorful*: `0.0` is gray, `1.0` is as vivid as possible.
//! - **Lightness** is *how bright*, as `(darkest, lightest)`: `0.0` is black,
//!   `1.0` is white.
//!
//! Colors for the background come from [`Rgba`]: `Rgba::WHITE`,
//! `Rgba::rgb(255, 248, 231)`, or a hex code with [`Rgba::from_hex`].
//!
//! # Keep a user's random avatar forever
//!
//! This is what most apps want: pick a random avatar when a user signs up,
//! then show that same avatar every time.
//!
//! A seed only stands for the *random choices*. Size, colors and style are
//! separate, so rebuild with the same settings. The easiest way is to set up
//! one builder and reuse it:
//!
//! ```
//! use ferrav::{Avatar, Mode, Pixel};
//!
//! // Your app's avatar settings, written once.
//! let settings = Avatar::builder().size(96).style(Pixel::default());
//!
//! // When a user signs up:
//! let avatar = settings.clone().text("sagar").mode(Mode::Random).build()?;
//! let seed = avatar.seed(); // save this with the user
//!
//! // Every time you show their avatar:
//! let same = settings.seed(seed).build()?;
//! assert_eq!(avatar, same);
//! # Ok::<(), ferrav::Error>(())
//! ```
//!
//! # Make your own style
//!
//! A style is anything that implements the [`Style`] trait: one function that
//! returns a list of shapes. Here's one that draws a single circle:
//!
//! ```
//! use ferrav::{Avatar, Palette, Point, Rng, Shape, Style};
//!
//! #[derive(Debug)]
//! struct Dot;
//!
//! impl Style for Dot {
//!     fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
//!         let radius = 0.2 + rng.next_f32() * 0.3; // a random size
//!         vec![Shape::circle(Point::new(0.5, 0.5), radius, palette.mid)]
//!     }
//! }
//!
//! let avatar = Avatar::builder().text("sagar").style(Dot).build()?;
//! # Ok::<(), ferrav::Error>(())
//! ```
//!
//! Three rules for a style:
//!
//! 1. Draw inside a square from `(0, 0)` (top-left) to `(1, 1)` (bottom-right).
//!    ferrav scales it to the real size and adds the padding.
//! 2. Take every random choice from `rng`. That's what makes seeds work.
//! 3. Pick colors from the [`Palette`], so the user's color settings apply.
//!
//! # Errors
//!
//! Settings that are out of range give an [`Error`] with a readable message.
//! They never crash your program:
//!
//! ```
//! use ferrav::Avatar;
//!
//! match Avatar::builder().size(5).build() {
//!     Ok(avatar) => println!("made a {} px avatar", avatar.size()),
//!     Err(error) => println!("could not make it: {error}"),
//! }
//! ```
//!
//! [`Avatar::new`], [`Avatar::random`] and [`Avatar::from_seed`] always use
//! valid settings, so they can't fail.
//!
//! # How it works
//!
//! ```text
//! text ──> seed ──> random choices ──> shapes (a Scene) ──┬──> SVG
//!                   (palette + style)                     ├──> RGBA pixels
//!                                                         └──> PNG
//! ```
//!
//! The text becomes a seed. The seed drives every random choice: the colors
//! (a [`Palette`]) and what the [`Style`] draws. The result is a [`Scene`], a
//! plain list of colored shapes, which is turned into SVG, pixels or PNG.
//! You can look at it with [`Avatar::scene`], or draw your own scenes with
//! [`render_svg`], [`render_rgba`] and `encode_png`.
//!
//! # Cargo features
//!
//! - `png` (on by default): PNG output through `Avatar::to_png` and
//!   `encode_png`, using the small pure-Rust `miniz_oxide` crate.
//!
//! For SVG only, with zero dependencies:
//!
//! ```toml
//! [dependencies]
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