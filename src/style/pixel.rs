//! A GitHub-style look: a grid of squares, mirrored left to right.

use super::Style;
use crate::color::{Palette, Rgba};
use crate::geometry::Point;
use crate::rng::Rng;
use crate::scene::Shape;

/// GitHub-style avatars: random squares on a grid, mirrored horizontally.
///
/// ```
/// // A 7×7 grid where about a third of the squares are filled.
/// let style = ferrav::Pixel {
///     grid: 7,
///     density: 0.35,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    /// Squares per row and column, from 2 to [`Pixel::MAX_GRID`].
    /// Smaller values are treated as 2 and larger ones as `MAX_GRID`.
    pub grid: u32,

    /// Chance that each square is filled, from 0.0 to 1.0.
    /// Values outside that range are clamped to it.
    pub density: f32,
}

impl Pixel {
    /// The largest grid allowed. Bigger values are treated as this.
    ///
    /// Without a limit, one huge `grid` value could make drawing take
    /// forever or run out of memory.
    pub const MAX_GRID: u32 = 64;
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            grid: 5,
            density: 0.5,
        }
    }
}

impl Style for Pixel {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let grid = self.grid.clamp(2, Self::MAX_GRID);
        let density = self.density.clamp(0.0, 1.0);

        let size = 1.0 / grid as f32;
        let fill = palette.mid;

        // Generate only the left half (plus the middle column) and mirror it.
        let half = grid.div_ceil(2);

        // Safe: grid is at most MAX_GRID, so grid * grid can't overflow.
        let mut shapes = Vec::with_capacity((grid * grid) as usize);

        for row in 0..grid {
            for col in 0..half {
                if !rng.chance(density) {
                    continue;
                }

                let mirror = grid - 1 - col;

                shapes.push(square(col, row, size, fill));

                if mirror != col {
                    shapes.push(square(mirror, row, size, fill));
                }
            }
        }

        // Guarantee a visible avatar.
        if shapes.is_empty() {
            shapes.push(square(grid / 2, grid / 2, size, fill));
        }

        shapes
    }
}

/// Creates one filled grid square at (`col`, `row`).
fn square(col: u32, row: u32, size: f32, fill: Rgba) -> Shape {
    let x = col as f32 * size;
    let y = row as f32 * size;

    Shape::polygon(
        vec![
            Point::new(x, y),
            Point::new(x + size, y),
            Point::new(x + size, y + size),
            Point::new(x, y + size),
        ],
        fill,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorOptions;

    fn palette() -> Palette {
        Palette::new(0.0, &ColorOptions::default())
    }

    #[test]
    fn never_empty() {
        let style = Pixel {
            grid: 5,
            density: 0.0,
        };

        let shapes = style.draw(&mut Rng::new(3), &palette());

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn full_density_fills_grid() {
        let style = Pixel {
            grid: 7,
            density: 1.0,
        };

        let shapes = style.draw(&mut Rng::new(3), &palette());

        assert_eq!(shapes.len(), 49);
    }

    #[test]
    fn clamps_invalid_density() {
        let style = Pixel {
            grid: 5,
            density: 10.0,
        };

        let shapes = style.draw(&mut Rng::new(3), &palette());

        assert_eq!(shapes.len(), 25);
    }

    #[test]
    fn grid_is_clamped() {
        let tiny = Pixel {
            grid: 0,
            density: 1.0,
        };
        assert_eq!(tiny.draw(&mut Rng::new(3), &palette()).len(), 4); // 2×2

        let huge = Pixel {
            grid: u32::MAX,
            density: 1.0,
        };
        let max = (Pixel::MAX_GRID * Pixel::MAX_GRID) as usize;
        assert_eq!(huge.draw(&mut Rng::new(3), &palette()).len(), max); // 64×64
    }
}