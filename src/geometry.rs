//! Points, cells, rotations, and the small amount of math needed to place shapes.

/// A point in 2D. `x` grows to the right and `y` grows *downwards*,
/// the same as on screens and in SVG.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// Horizontal position.
    pub x: f32,

    /// Vertical position (down is positive).
    pub y: f32,
}

impl Point {
    /// Creates a point.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Rotates a point inside the unit square (0 to 1 on both axes)
    /// clockwise around the square's center.
    pub const fn rotate_in_unit(self, rotation: Rotation) -> Self {
        let Point { x, y } = self;

        match rotation {
            Rotation::Zero => Point::new(x, y),
            Rotation::Clockwise90 => Point::new(1.0 - y, x),
            Rotation::Clockwise180 => Point::new(1.0 - x, 1.0 - y),
            Rotation::Clockwise270 => Point::new(y, 1.0 - x),
        }
    }
}

/// Clockwise rotations in 90° steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation.
    Zero,

    /// A quarter turn clockwise.
    Clockwise90,

    /// A half turn.
    Clockwise180,

    /// Three quarter turns clockwise.
    Clockwise270,
}

impl Rotation {
    /// Creates a rotation from clockwise quarter turns.
    ///
    /// Values wrap around:
    /// - `4` becomes [`Rotation::Zero`]
    /// - `5` becomes [`Rotation::Clockwise90`]
    pub const fn from_quarter_turns(turns: u8) -> Self {
        match turns % 4 {
            0 => Self::Zero,
            1 => Self::Clockwise90,
            2 => Self::Clockwise180,
            _ => Self::Clockwise270,
        }
    }
}

/// A square area: its top-left corner and the length of its side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    /// Left edge.
    pub x: f32,

    /// Top edge.
    pub y: f32,

    /// Width and height.
    pub size: f32,
}

impl Cell {
    /// Creates a cell.
    pub const fn new(x: f32, y: f32, size: f32) -> Self {
        Self { x, y, size }
    }

    /// Returns the center point of this cell.
    pub const fn center(&self) -> Point {
        Point::new(self.x + self.size / 2.0, self.y + self.size / 2.0)
    }

    /// Transforms a unit-square point into this cell.
    ///
    /// The transformation order is:
    /// 1. Rotate inside the unit square
    /// 2. Scale by the cell size
    /// 3. Move to the cell's position
    pub const fn transform_point(&self, point: Point, rotation: Rotation) -> Point {
        let point = point.rotate_in_unit(rotation);

        Point::new(self.x + point.x * self.size, self.y + point.y * self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fails the test if the two points are more than a tiny distance apart.
    fn assert_close(a: Point, b: Point) {
        assert!((a.x - b.x).abs() < 1e-6, "x differs: {} != {}", a.x, b.x);
        assert!((a.y - b.y).abs() < 1e-6, "y differs: {} != {}", a.y, b.y);
    }

    #[test]
    fn rotates_all_directions() {
        let point = Point::new(0.0, 0.0);

        assert_close(point.rotate_in_unit(Rotation::Zero), Point::new(0.0, 0.0));
        assert_close(
            point.rotate_in_unit(Rotation::Clockwise90),
            Point::new(1.0, 0.0),
        );
        assert_close(
            point.rotate_in_unit(Rotation::Clockwise180),
            Point::new(1.0, 1.0),
        );
        assert_close(
            point.rotate_in_unit(Rotation::Clockwise270),
            Point::new(0.0, 1.0),
        );
    }

    #[test]
    fn quarter_turn_conversion_wraps() {
        assert_eq!(Rotation::from_quarter_turns(4), Rotation::Zero);
        assert_eq!(Rotation::from_quarter_turns(5), Rotation::Clockwise90);
    }

    #[test]
    fn transforms_point_into_cell() {
        let cell = Cell::new(10.0, 20.0, 5.0);

        assert_close(
            cell.transform_point(Point::new(1.0, 1.0), Rotation::Zero),
            Point::new(15.0, 25.0),
        );
    }

    #[test]
    fn transforms_with_rotation() {
        let cell = Cell::new(10.0, 20.0, 10.0);

        assert_close(
            cell.transform_point(Point::new(0.0, 0.0), Rotation::Clockwise90),
            Point::new(20.0, 20.0),
        );
    }

    #[test]
    fn finds_cell_center() {
        let cell = Cell::new(10.0, 20.0, 10.0);

        assert_close(cell.center(), Point::new(15.0, 25.0));
    }
}
