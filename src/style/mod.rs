//! Styles decide what an avatar looks like.
//!
//! ferrav ships two: [`Geometric`] (the default, a 4×4 symmetric look) and
//! [`Pixel`] (GitHub-like). Implement [`Style`] to add your own.

mod geometric;
mod pixel;

pub use geometric::Geometric;
pub use pixel::Pixel;

use crate::color::Palette;
use crate::rng::Rng;
use crate::scene::Shape;

/// A drawing style: turns random decisions and a palette into shapes.
///
/// Draw inside the unit square, where (0, 0) is the top-left corner and
/// (1, 1) the bottom-right. ferrav scales your shapes to the final size and
/// padding. Take every random decision from `rng`, never from anywhere else,
/// so the same seed always gives the same picture.
///
/// The `Debug + Send + Sync` part lets avatars be printed and shared
/// between threads. `#[derive(Debug)]` on your struct is all it takes.
pub trait Style: std::fmt::Debug + Send + Sync {
    /// Returns the shapes for one avatar.
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape>;
}
