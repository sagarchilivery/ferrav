//! Turns a [`Scene`] into SVG text.

use std::fmt::{self, Write};

use crate::color::Rgba;
use crate::scene::{Geometry, Scene};

/// Renders a scene as a complete SVG document.
pub fn render_svg(scene: &Scene) -> String {
    // A rough guess of the final length, so the string rarely has to grow.
    let mut svg = String::with_capacity(128 + scene.shapes.len() * 96);
    write_svg(&mut svg, scene).expect("writing to a String never fails");
    svg
}

/// Writes the whole document. It returns `fmt::Result` so every `write!`
/// can use `?` instead of its own `.unwrap()`.
fn write_svg(out: &mut String, scene: &Scene) -> fmt::Result {
    let size = scene.size;
    write!(
        out,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 {size} {size}">"#
    )?;

    if let Some(bg) = scene.background {
        out.push_str(r#"<rect width="100%" height="100%" "#);
        write_paint(out, bg)?;
        out.push_str("/>");
    }

    for shape in &scene.shapes {
        match &shape.geometry {
            Geometry::Polygon(points) => {
                out.push_str(r#"<polygon points=""#);
                for (index, point) in points.iter().enumerate() {
                    if index > 0 {
                        out.push(' ');
                    }
                    write!(out, "{},{}", Num(point.x), Num(point.y))?;
                }
                out.push_str("\" ");
                write_paint(out, shape.fill)?;
                out.push_str("/>");
            }
            Geometry::Circle { center, radius } => {
                write!(
                    out,
                    r#"<circle cx="{}" cy="{}" r="{}" "#,
                    Num(center.x),
                    Num(center.y),
                    Num(*radius)
                )?;
                write_paint(out, shape.fill)?;
                out.push_str("/>");
            }
        }
    }

    out.push_str("</svg>");
    Ok(())
}

/// Writes `fill="#rrggbb"`, plus `fill-opacity="…"` for see-through colors.
fn write_paint(out: &mut String, color: Rgba) -> fmt::Result {
    write!(out, r#"fill="{color}""#)?;
    if color.a != 255 {
        write!(out, r#" fill-opacity="{}""#, Num(color.opacity()))?;
    }
    Ok(())
}

/// A number written compactly for SVG: rounded to 2 decimal places, so
/// `12.3456` is written as `12.35` and `5.0` as `5`.
///
/// Implementing `Display` lets `write!` format it straight into the output,
/// without creating a temporary `String` for every number.
struct Num(f32);

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `+ 0.0` turns -0.0 into 0.0, so we never write "-0".
        let rounded = (self.0 * 100.0).round() / 100.0 + 0.0;
        write!(f, "{rounded}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::scene::Shape;

    #[test]
    fn writes_shapes_and_background() {
        let scene = Scene {
            size: 10,
            background: Some(Rgba::WHITE),
            shapes: vec![Shape::circle(Point::new(5.0, 5.0), 2.5, Rgba::BLACK)],
        };

        assert_eq!(
            render_svg(&scene),
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10"><rect width="100%" height="100%" fill="#ffffff"/><circle cx="5" cy="5" r="2.5" fill="#000000"/></svg>"##
        );
    }

    #[test]
    fn writes_polygons() {
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(0.0, 10.0),
        ];
        let scene = Scene {
            size: 10,
            background: None,
            shapes: vec![Shape::polygon(triangle, Rgba::BLACK)],
        };

        assert_eq!(
            render_svg(&scene),
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10"><polygon points="0,0 10,0 0,10" fill="#000000"/></svg>"##
        );
    }

    #[test]
    fn writes_transparent_colors() {
        let scene = Scene {
            size: 10,
            background: Some(Rgba::new(255, 0, 0, 128)),
            shapes: vec![],
        };

        assert!(render_svg(&scene).contains(r##"fill="#ff0000" fill-opacity="0.5""##));
    }

    #[test]
    fn numbers_are_short() {
        assert_eq!(Num(5.0).to_string(), "5");
        assert_eq!(Num(12.3456).to_string(), "12.35");
        assert_eq!(Num(-0.001).to_string(), "0");
    }
}