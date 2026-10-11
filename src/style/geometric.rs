//! The default look: a 4×4 grid with four-way rotational symmetry.

use super::Style;
use crate::color::{Palette, Rgba};
use crate::geometry::{Cell, Point, Rotation};
use crate::rng::Rng;
use crate::scene::Shape;

/// Symmetric geometric avatars: one shape repeated around the corners, one
/// around the sides, one in the center, each rotated so the picture looks
/// the same after a quarter turn.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Geometric;

/// A shape drawn in the unit square, before it is placed in a cell.
#[derive(Debug, Clone, Copy)]
enum Template {
    Polygon(&'static [(f32, f32)]),
    Circle { cx: f32, cy: f32, r: f32 },
}

/// Shapes for the corner and side cells.
const OUTER: [Template; 6] = [
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]), // corner triangle
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.5), (0.0, 1.0)]), // arrow
    Template::Polygon(&[(0.5, 0.0), (1.0, 0.5), (0.5, 1.0), (0.0, 0.5)]), // diamond
    Template::Circle {
        cx: 0.5,
        cy: 0.5,
        r: 0.3,
    }, // dot
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 0.5)]), // half bar
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 0.5), (0.0, 1.0)]), // notch
];

/// Shapes for the four center cells. In these cells the point (1, 1) is the
/// middle of the whole avatar, so these shapes meet in the middle.
const CENTER: [Template; 6] = [
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]), // full square
    Template::Polygon(&[(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]), // diamond in the middle
    Template::Polygon(&[(0.0, 0.0), (0.7, 0.3), (1.0, 1.0), (0.3, 0.7)]), // kite
    Template::Polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]), // pinwheel blade
    Template::Polygon(&[(0.25, 0.25), (0.75, 0.25), (0.75, 0.75), (0.25, 0.75)]), // small square
    Template::Circle {
        cx: 1.0,
        cy: 1.0,
        r: 0.6,
    }, // one big dot in the middle
];

// Each zone is 4 cells, as (column, row) in the 4×4 grid. Turning the whole
// picture 90° clockwise moves each cell to the next one in its list, which
// is why the next cell also turns its shape one more quarter turn.
const CORNER_CELLS: [(f32, f32); 4] = [(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)];
const SIDE_CELLS_A: [(f32, f32); 4] = [(1.0, 0.0), (3.0, 1.0), (2.0, 3.0), (0.0, 2.0)];
const SIDE_CELLS_B: [(f32, f32); 4] = [(2.0, 0.0), (3.0, 2.0), (1.0, 3.0), (0.0, 1.0)];
const CENTER_CELLS: [(f32, f32); 4] = [(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)];

/// Each grid cell is a quarter of the unit square.
const CELL: f32 = 0.25;

impl Style for Geometric {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        // 1. Pick three different colors, one per zone. A palette always has
        //    exactly 5 colors (its type is [Rgba; 5]), so three removes are safe.
        let mut colors = palette.all().to_vec();
        let side_color = colors.remove(rng.below(colors.len()));
        let corner_color = colors.remove(rng.below(colors.len()));
        let center_color = colors.remove(rng.below(colors.len()));

        // 2. Pick a shape and a starting rotation for each zone.
        let side_template = *rng.pick(&OUTER);
        let side_turns = rng.below(4) as u8;
        let corner_template = *rng.pick(&OUTER);
        let corner_turns = rng.below(4) as u8;
        let center_template = *rng.pick(&CENTER);

        // 3. Stamp each shape into its four cells.
        let mut shapes = Vec::with_capacity(16);
        stamp(
            &mut shapes,
            &SIDE_CELLS_A,
            side_template,
            side_turns,
            side_color,
        );
        stamp(
            &mut shapes,
            &SIDE_CELLS_B,
            side_template,
            side_turns,
            side_color,
        );
        stamp(
            &mut shapes,
            &CORNER_CELLS,
            corner_template,
            corner_turns,
            corner_color,
        );
        // Center shapes always start unrotated: they're drawn to meet in the middle.
        stamp(
            &mut shapes,
            &CENTER_CELLS,
            center_template,
            0,
            center_color,
        );
        shapes
    }
}

/// Draws `template` in each of the four cells, one more quarter turn each time.
fn stamp(
    shapes: &mut Vec<Shape>,
    cells: &[(f32, f32); 4],
    template: Template,
    first_turns: u8,
    fill: Rgba,
) {
    for (index, &(column, row)) in cells.iter().enumerate() {
        let cell = Cell::new(column * CELL, row * CELL, CELL);
        shapes.push(place(template, cell, first_turns + index as u8, fill));
    }
}

/// Turns a template into a real shape inside `cell`.
fn place(template: Template, cell: Cell, quarter_turns: u8, fill: Rgba) -> Shape {
    let rotation = Rotation::from_quarter_turns(quarter_turns);

    match template {
        Template::Polygon(points) => {
            let points = points
                .iter()
                .map(|&(x, y)| cell.transform_point(Point::new(x, y), rotation))
                .collect();
            Shape::polygon(points, fill)
        }
        Template::Circle { cx, cy, r } => {
            // The center rotates like any other point. Rotating doesn't change
            // a circle's size, so the radius only needs scaling.
            let center = cell.transform_point(Point::new(cx, cy), rotation);
            Shape::circle(center, r * cell.size, fill)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorOptions;
    use crate::scene::Geometry;

    #[test]
    fn draws_four_shapes_per_zone() {
        let palette = Palette::new(200.0, &ColorOptions::default());
        let shapes = Geometric.draw(&mut Rng::new(1), &palette);
        assert_eq!(shapes.len(), 16); // 4 zones x 4 cells
    }

    #[test]
    fn center_dots_meet_in_the_middle() {
        // The big center dot is drawn once per center cell. If circle centers
        // weren't rotated with their cell, the four copies would land in four
        // different places instead of on top of each other.
        let mut shapes = Vec::new();
        stamp(&mut shapes, &CENTER_CELLS, CENTER[5], 0, Rgba::BLACK);

        for shape in &shapes {
            let Geometry::Circle { center, .. } = &shape.geometry else {
                panic!("expected a circle");
            };
            assert!((center.x - 0.5).abs() < 1e-6, "x was {}", center.x);
            assert!((center.y - 0.5).abs() < 1e-6, "y was {}", center.y);
        }
    }
}