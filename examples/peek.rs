//! A quick look at the output before the public API exists.
//! Run with: cargo run --example peek

use ferrav::{
    ColorOptions, Geometric, Palette, Point, Rng, Scene, Style, encode_png, render_rgba, render_svg,
};

fn main() {
    // Same seed -> same picture. Change 42 to see others.
    let mut rng = Rng::new(42);
    let palette = Palette::random(&mut rng, &ColorOptions::default());

    // The style draws in a 0..1 square; stretch it to 128 px with 10 px of padding.
    let shapes = Geometric
        .draw(&mut rng, &palette)
        .into_iter()
        .map(|shape| shape.transform(Point::new(10.0, 10.0), 108.0))
        .collect();

    let scene = Scene {
        size: 128,
        background: None,
        shapes,
    };
    std::fs::write("peek.svg", render_svg(&scene)).unwrap();
    std::fs::write("peek.png", encode_png(128, 128, &render_rgba(&scene))).unwrap();
    println!("wrote peek.svg and peek.png - open them");
}