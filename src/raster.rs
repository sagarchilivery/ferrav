//! Turns a [`Scene`] into pixels (RGBA bytes) with smooth, anti-aliased edges.
//!
//! The idea: every pixel holds a 4×4 grid of tiny sample points. At each
//! point we paint the shapes in order, like stacking sheets of paper. The
//! pixel's final color is the average of its 16 points, weighted by each
//! point's opacity, so a pixel that is half inside a shape gets a color
//! halfway between the shape and what's behind it. That's anti-aliasing.

use crate::color::Rgba;
use crate::geometry::Point;
use crate::scene::{Geometry, Scene};

/// Sample points per pixel along each axis: 4 × 4 = 16 per pixel.
/// More samples give smoother edges but take longer.
const SAMPLES: u32 = 4;

/// Renders a scene into RGBA bytes: 4 bytes per pixel, left to right, top to bottom.
///
/// Pixel `(x, y)` starts at index `(y * size + x) * 4`. The result is
/// `size × size × 4` bytes long.
pub fn render_rgba(scene: &Scene) -> Vec<u8> {
    // Work out each shape's bounding box once, so far-away shapes are skipped quickly.
    let boxes: Vec<(Point, Point)> = scene
        .shapes
        .iter()
        .map(|shape| bounds(&shape.geometry))
        .collect();

    let size = scene.size;
    let mut pixels = Vec::with_capacity(size as usize * size as usize * 4);

    for y in 0..size {
        for x in 0..size {
            pixels.extend_from_slice(&pixel(scene, &boxes, x, y));
        }
    }

    pixels
}

/// The final color of pixel (x, y): the average of its sample points.
fn pixel(scene: &Scene, boxes: &[(Point, Point)], x: u32, y: u32) -> [u8; 4] {
    let step = 1.0 / SAMPLES as f32;

    // Each sample's color is weighted by its opacity, so see-through samples
    // count less. (This weighting is called "premultiplied alpha".)
    let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);

    for sy in 0..SAMPLES {
        for sx in 0..SAMPLES {
            let point = Point::new(
                x as f32 + (sx as f32 + 0.5) * step,
                y as f32 + (sy as f32 + 0.5) * step,
            );

            let color = color_at(scene, boxes, point);
            let alpha = color.opacity();

            r += f32::from(color.r) * alpha;
            g += f32::from(color.g) * alpha;
            b += f32::from(color.b) * alpha;
            a += alpha;
        }
    }

    if a <= 0.0 {
        return [0, 0, 0, 0];
    }

    let samples = (SAMPLES * SAMPLES) as f32;
    // Dividing by the total weight undoes the weighting ("unpremultiplies").
    let unpremultiply = |value: f32| (value / a).round().clamp(0.0, 255.0) as u8;

    [
        unpremultiply(r),
        unpremultiply(g),
        unpremultiply(b),
        (a / samples * 255.0).round() as u8,
    ]
}

/// The color at one exact point: start with the background, then paint
/// every shape that contains the point on top, in order.
fn color_at(scene: &Scene, boxes: &[(Point, Point)], point: Point) -> Rgba {
    let mut color = scene.background.unwrap_or(Rgba::TRANSPARENT);

    for (shape, &(min, max)) in scene.shapes.iter().zip(boxes) {
        let in_box = point.x >= min.x && point.x < max.x && point.y >= min.y && point.y < max.y;

        if in_box && contains(&shape.geometry, point) {
            color = over(shape.fill, color);
        }
    }

    color
}

/// Is `point` inside the shape?
fn contains(geometry: &Geometry, point: Point) -> bool {
    match geometry {
        Geometry::Polygon(points) => point_in_polygon(point, points),

        Geometry::Circle { center, radius } => {
            let dx = point.x - center.x;
            let dy = point.y - center.y;

            dx * dx + dy * dy <= radius * radius
        }
    }
}

/// The "ray casting" test: walk from `point` to the right and count how many
/// edges of the polygon we cross. An odd count means the point is inside.
fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let Some(&last) = polygon.last() else {
        return false;
    };

    let mut inside = false;
    let mut previous = last;

    for &current in polygon {
        let crosses = (current.y > point.y) != (previous.y > point.y);

        if crosses {
            // x where this edge crosses the horizontal line through `point`
            let x = current.x
                + (point.y - current.y) * (previous.x - current.x) / (previous.y - current.y);

            if point.x < x {
                inside = !inside;
            }
        }

        previous = current;
    }

    inside
}

/// The smallest rectangle (top-left, bottom-right) that holds the shape.
fn bounds(geometry: &Geometry) -> (Point, Point) {
    match geometry {
        Geometry::Polygon(points) => {
            let mut min = Point::new(f32::INFINITY, f32::INFINITY);
            let mut max = Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

            for &point in points {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
            }

            (min, max)
        }

        Geometry::Circle { center, radius } => (
            Point::new(center.x - radius, center.y - radius),
            Point::new(center.x + radius, center.y + radius),
        ),
    }
}

/// Paints `top` over `bottom` (standard "source over" blending).
fn over(top: Rgba, bottom: Rgba) -> Rgba {
    if top.a == 255 {
        return top; // solid paint simply covers what's below
    }

    let top_alpha = top.opacity();
    let bottom_alpha = bottom.opacity();
    let out_alpha = top_alpha + bottom_alpha * (1.0 - top_alpha);

    if out_alpha <= 0.0 {
        return Rgba::TRANSPARENT;
    }

    let blend = |t: u8, b: u8| {
        let value = (f32::from(t) * top_alpha + f32::from(b) * bottom_alpha * (1.0 - top_alpha))
            / out_alpha;
        value.round() as u8
    };

    Rgba::new(
        blend(top.r, bottom.r),
        blend(top.g, bottom.g),
        blend(top.b, bottom.b),
        (out_alpha * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Shape;

    /// A rectangle from x0 to x1, 4 pixels tall.
    fn rect(x0: f32, x1: f32) -> Vec<Point> {
        vec![
            Point::new(x0, 0.0),
            Point::new(x1, 0.0),
            Point::new(x1, 4.0),
            Point::new(x0, 4.0),
        ]
    }

    #[test]
    fn point_in_polygon_works() {
        let square = rect(0.0, 4.0);
        assert!(point_in_polygon(Point::new(2.0, 2.0), &square));
        assert!(!point_in_polygon(Point::new(5.0, 2.0), &square));
        assert!(!point_in_polygon(Point::new(2.0, 2.0), &[]));
    }

    #[test]
    fn half_transparent_black_over_white_is_gray() {
        let gray = over(Rgba::new(0, 0, 0, 128), Rgba::WHITE);
        assert_eq!(gray, Rgba::new(127, 127, 127, 255));
    }

    #[test]
    fn a_full_size_square_fills_every_pixel() {
        let scene = Scene {
            size: 4,
            background: None,
            shapes: vec![Shape::polygon(rect(0.0, 4.0), Rgba::rgb(1, 2, 3))],
        };
        let pixels = render_rgba(&scene);
        assert_eq!(pixels.len(), 4 * 4 * 4);
        assert!(pixels.chunks_exact(4).all(|px| px == [1, 2, 3, 255]));
    }

    #[test]
    fn half_covered_pixels_are_mixed() {
        // A black rectangle covering the left half of pixel column 1, on white.
        let scene = Scene {
            size: 4,
            background: Some(Rgba::WHITE),
            shapes: vec![Shape::polygon(rect(0.0, 1.5), Rgba::BLACK)],
        };
        let pixels = render_rgba(&scene);
        assert_eq!(&pixels[0..4], &[0, 0, 0, 255]); // pixel 0: fully covered
        assert_eq!(&pixels[4..8], &[128, 128, 128, 255]); // pixel 1: half covered
        assert_eq!(&pixels[8..12], &[255, 255, 255, 255]); // pixel 2: not covered
    }

    #[test]
    fn touching_shapes_leave_no_seam() {
        // Two same-colored rectangles meeting halfway through a pixel.
        let color = Rgba::rgb(9, 9, 9);
        let scene = Scene {
            size: 4,
            background: Some(Rgba::WHITE),
            shapes: vec![
                Shape::polygon(rect(0.0, 1.5), color),
                Shape::polygon(rect(1.5, 4.0), color),
            ],
        };
        let pixels = render_rgba(&scene);
        assert!(pixels.chunks_exact(4).all(|px| px == [9, 9, 9, 255]));
    }
}