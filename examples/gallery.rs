//! Every feature at once: writes ./out/gallery.html, one page that shows
//! every kind of avatar side by side.
//!
//! Run with: cargo run --example gallery
//! Then open out/gallery.html in your browser.

use std::fs;

use ferrav::{
    Avatar, ColorOptions, Geometric, Mode, Palette, Pixel, Point, Rgba, Rng, Shape, Style,
};

/// A small custom style: rings in the palette's colors, biggest first.
#[derive(Debug)]
struct Rings {
    count: u32,
}

impl Style for Rings {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let colors = palette.all();
        (0..self.count)
            .map(|i| {
                let radius = 0.5 * (self.count - i) as f32 / self.count as f32;
                Shape::circle(Point::new(0.5, 0.5), radius, *rng.pick(&colors))
            })
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // Every avatar goes in here with a label, to build the page at the end.
    let mut gallery: Vec<(String, Avatar)> = Vec::new();

    // Deterministic: same text, same avatar.
    gallery.push(("Deterministic: \"sagar\"".into(), Avatar::new("sagar")));
    gallery.push(("Same text again".into(), Avatar::new("sagar")));
    gallery.push(("Different text: \"ravi\"".into(), Avatar::new("ravi")));

    // Random, then restored from its seed.
    let random = Avatar::random("sagar");
    let restored = Avatar::from_seed(random.seed());
    gallery.push((format!("Random (seed {})", random.seed()), random));
    gallery.push(("Restored from that seed".into(), restored));

    // Styles.
    let roomy = Avatar::builder()
        .text("sagar")
        .style(Geometric)
        .padding(0.2)
        .build()?;
    gallery.push(("Geometric, padding 0.2".into(), roomy));

    for (grid, density) in [(5, 0.5), (7, 0.35), (9, 0.6)] {
        let avatar = Avatar::builder()
            .text("sagar")
            .style(Pixel { grid, density })
            .build()?;
        gallery.push((format!("Pixel: grid {grid}, density {density}"), avatar));
    }

    let random_pixel = Avatar::builder()
        .text("sagar")
        .mode(Mode::Random)
        .style(Pixel::default())
        .build()?;
    gallery.push(("Pixel, random mode".into(), random_pixel));

    // Colors.
    let color_examples = [
        (
            "Only blue (hue 210)",
            ColorOptions {
                hues: vec![210.0],
                ..Default::default()
            },
        ),
        (
            "Vivid",
            ColorOptions {
                saturation: 0.9,
                lightness: (0.35, 0.65),
                ..Default::default()
            },
        ),
        (
            "Pastel",
            ColorOptions {
                saturation: 0.45,
                lightness: (0.7, 0.9),
                ..Default::default()
            },
        ),
        (
            "Grayscale",
            ColorOptions {
                saturation: 0.0,
                ..Default::default()
            },
        ),
    ];
    for (label, colors) in color_examples {
        let avatar = Avatar::builder().text("sagar").colors(colors).build()?;
        gallery.push((label.into(), avatar));
    }

    // Backgrounds.
    let on_white = Avatar::builder()
        .text("sagar")
        .background(Rgba::WHITE)
        .build()?;
    gallery.push(("White background".into(), on_white));

    let on_dark = Avatar::builder()
        .text("sagar")
        .background(Rgba::from_hex("#1e1e2e")?)
        .colors(ColorOptions {
            lightness: (0.6, 0.9),
            ..Default::default()
        })
        .build()?;
    gallery.push(("Dark background".into(), on_dark));

    // One builder reused for several users, like in a real app.
    let settings = Avatar::builder()
        .style(Pixel::default())
        .background(Rgba::WHITE);
    for name in ["alice", "bob", "charlie"] {
        let avatar = settings.clone().text(name).build()?;
        gallery.push((format!("Shared settings: {name}"), avatar));
    }

    // A custom style, defined at the top of this file.
    let rings = Avatar::builder()
        .text("sagar")
        .style(Rings { count: 4 })
        .build()?;
    gallery.push(("Custom style: Rings".into(), rings));

    // Build one HTML page with every avatar on it.
    let mut html = String::from(
        r#"<!doctype html>
<meta charset="utf-8">
<title>ferrav gallery</title>
<style>
  body { font-family: system-ui, sans-serif; background: #f4f4f5; margin: 2rem; }
  .grid { display: flex; flex-wrap: wrap; gap: 1rem; }
  figure { margin: 0; background: #fff; padding: 1rem; border-radius: 12px; width: 150px; text-align: center; }
  figure svg { width: 128px; height: 128px; }
  figcaption { font-size: 12px; color: #555; margin-top: 0.5rem; }
</style>
<h1>ferrav gallery</h1>
<div class="grid">
"#,
    );
    for (label, avatar) in &gallery {
        let svg = avatar.to_svg();
        html.push_str(&format!(
            "<figure>{svg}<figcaption>{label}</figcaption></figure>\n"
        ));
    }
    html.push_str("</div>\n");
    fs::write("out/gallery.html", html)?;

    println!("wrote {} avatars to out/gallery.html", gallery.len());
    Ok(())
}
