//! The [`Scene`]: a plain list of colored shapes.
//!
//! Styles produce shapes; the avatar builder collects them into a scene;
//! renderers (SVG, pixels) consume scenes. Keeping this layer in the middle
//! makes every part swappable.

use crate::color::Rgba;
use crate::geometry::Point;

/// The outline of one shape.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Geometry {
    /// A closed polygon through these points, in order.
    Polygon(Vec<Point>),

    /// A circle.
    Circle {
        /// Center point.
        center: Point,

        /// Radius.
        radius: f32,
    },
}

impl Geometry {
    /// Scales this geometry by `scale` (around the origin), then moves it by `offset`.
    ///
    /// A negative scale mirrors the shape; a circle's radius always stays positive.
    pub fn transform(self, offset: Point, scale: f32) -> Self {
        let map = |p: Point| Point::new(offset.x + p.x * scale, offset.y + p.y * scale);

        match self {
            Geometry::Polygon(points) => Geometry::Polygon(points.into_iter().map(map).collect()),

            Geometry::Circle { center, radius } => Geometry::Circle {
                center: map(center),
                radius: radius * scale.abs(),
            },
        }
    }

    /// Scales this geometry around the origin.
    pub fn scale(self, scale: f32) -> Self {
        self.transform(Point::new(0.0, 0.0), scale)
    }

    /// Moves this geometry by an offset.
    pub fn translate(self, offset: Point) -> Self {
        self.transform(offset, 1.0)
    }
}

/// One filled shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// The outline.
    pub geometry: Geometry,

    /// The fill color.
    pub fill: Rgba,
}

impl Shape {
    /// A filled polygon.
    pub fn polygon(points: Vec<Point>, fill: Rgba) -> Self {
        Self {
            geometry: Geometry::Polygon(points),
            fill,
        }
    }

    /// A filled circle.
    pub fn circle(center: Point, radius: f32, fill: Rgba) -> Self {
        Self {
            geometry: Geometry::Circle { center, radius },
            fill,
        }
    }

    /// Scales this shape by `scale` (around the origin), then moves it by `offset`.
    pub fn transform(self, offset: Point, scale: f32) -> Self {
        Self {
            geometry: self.geometry.transform(offset, scale),
            fill: self.fill,
        }
    }

    /// Scales this shape around the origin.
    pub fn scale(self, scale: f32) -> Self {
        Self {
            geometry: self.geometry.scale(scale),
            fill: self.fill,
        }
    }

    /// Moves this shape by an offset.
    pub fn translate(self, offset: Point) -> Self {
        Self {
            geometry: self.geometry.translate(offset),
            fill: self.fill,
        }
    }
}

/// Everything needed to draw one avatar.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    /// Width and height in pixels.
    pub size: u32,

    /// Background color. `None` means transparent.
    pub background: Option<Rgba>,

    /// Shapes painted in order (later shapes appear on top).
    pub shapes: Vec<Shape>,
}

impl Scene {
    /// Creates an empty scene with a transparent background.
    pub fn new(size: u32) -> Self {
        Self {
            size,
            background: None,
            shapes: Vec::new(),
        }
    }

    /// Adds a shape on top of the ones already in the scene.
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_scales_then_shifts() {
        let dot = Shape::circle(Point::new(0.5, 0.5), 0.5, Rgba::BLACK);
        let moved = dot.transform(Point::new(10.0, 10.0), 100.0);

        assert_eq!(
            moved.geometry,
            Geometry::Circle {
                center: Point::new(60.0, 60.0),
                radius: 50.0
            }
        );
    }

    #[test]
    fn scale_and_translate_helpers() {
        let square = Shape::polygon(
            vec![Point::new(0.0, 0.0), Point::new(1.0, 1.0)],
            Rgba::BLACK,
        );

        let scaled = square.clone().scale(10.0);
        assert_eq!(
            scaled.geometry,
            Geometry::Polygon(vec![Point::new(0.0, 0.0), Point::new(10.0, 10.0)])
        );

        let moved = square.translate(Point::new(5.0, 5.0));
        assert_eq!(
            moved.geometry,
            Geometry::Polygon(vec![Point::new(5.0, 5.0), Point::new(6.0, 6.0)])
        );
    }

    #[test]
    fn scene_adds_shapes() {
        let mut scene = Scene::new(256);
        scene.add(Shape::circle(Point::new(0.0, 0.0), 10.0, Rgba::BLACK));

        assert_eq!(scene.shapes.len(), 1);
        assert_eq!(scene.size, 256);
        assert_eq!(scene.background, None);
    }

    #[test]
    fn negative_scale_keeps_radius_positive() {
        let circle = Geometry::Circle {
            center: Point::new(1.0, 1.0),
            radius: 5.0,
        };
        let transformed = circle.transform(Point::new(0.0, 0.0), -2.0);

        assert_eq!(
            transformed,
            Geometry::Circle {
                center: Point::new(-2.0, -2.0),
                radius: 10.0
            }
        );
    }
}
