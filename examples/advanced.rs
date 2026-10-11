//! For advanced users: handling errors, looking inside an avatar, raw pixels,
//! and drawing a scene by hand with the low-level functions.
//!
//! Run with: cargo run --example advanced

use std::fs;

use ferrav::{
    Avatar, Error, Geometry, Mode, Point, Rgba, Scene, Shape, encode_png, render_rgba, render_svg,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // 1. Errors. Bad settings give an `Err` with a readable message, never a crash.
    if let Err(err) = Avatar::builder().size(5).build() {
        println!("error: {err}");
    }
    if let Err(err) = Rgba::from_hex("not a color") {
        println!("error: {err}");
    }
    // You can also check which kind of error it is.
    match Avatar::builder().padding(0.7).build() {
        Err(Error::InvalidPadding(value)) => println!("padding {value} is not allowed"),
        Err(other) => println!("error: {other}"),
        Ok(_) => println!("built fine"),
    }

    // 2. The seed behind a text.
    println!(
        "seed for \"sagar\": {}",
        Mode::Deterministic.seed_for("sagar")
    );

    // 3. Look inside an avatar: it's a list of colored shapes (a "scene").
    let avatar = Avatar::new("sagar");
    let scene = avatar.scene();
    println!(
        "\"sagar\" is {} shapes on a {}x{} canvas; the first three:",
        scene.shapes.len(),
        scene.size,
        scene.size
    );
    for shape in scene.shapes.iter().take(3) {
        match &shape.geometry {
            Geometry::Polygon(points) => {
                println!("  polygon, {} corners, color {}", points.len(), shape.fill)
            }
            Geometry::Circle { radius, .. } => {
                println!("  circle, radius {radius:.1}, color {}", shape.fill)
            }
            _ => println!("  another kind of shape"),
        }
    }

    // 4. Raw pixels: 4 bytes per pixel (red, green, blue, alpha), row by row.
    let pixels = avatar.to_rgba();
    let size = avatar.size() as usize;
    let (x, y) = (size / 2, size / 2);
    let i = (y * size + x) * 4;
    println!(
        "middle pixel: rgba({}, {}, {}, {})",
        pixels[i],
        pixels[i + 1],
        pixels[i + 2],
        pixels[i + 3]
    );

    // 5. Draw a scene by hand and render it. Coordinates here are in pixels.
    let mut scene = Scene::new(64);
    scene.background = Some(Rgba::WHITE);
    scene.add(Shape::circle(
        Point::new(32.0, 32.0),
        24.0,
        Rgba::from_hex("#3a7bd5")?,
    ));
    scene.add(Shape::polygon(
        vec![
            Point::new(18.0, 44.0),
            Point::new(46.0, 44.0),
            Point::new(32.0, 18.0),
        ],
        Rgba::new(255, 255, 255, 200), // white, slightly see-through
    ));
    fs::write("out/hand-made.svg", render_svg(&scene))?;
    fs::write(
        "out/hand-made.png",
        encode_png(64, 64, &render_rgba(&scene)),
    )?;

    println!("wrote out/hand-made.svg and out/hand-made.png");
    Ok(())
}
