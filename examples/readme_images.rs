//! Writes the images used in README.md into ./assets.
//!
//! Run with: cargo run --example readme_images
//! Then commit the `assets` folder so GitHub can show the images.

use std::fs;

use ferrav::{Avatar, ColorOptions, Pixel, Rgba};

/// Saves one avatar as assets/<name>.png.
fn save(name: &str, avatar: &Avatar) -> std::io::Result<()> {
    fs::write(format!("assets/{name}.png"), avatar.to_png())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("assets")?;
    let base = Avatar::builder().size(192);

    // Top of the README: one avatar per text.
    let names = [
        ("sagar", "sagar"),
        ("ferrav", "ferrav"),
        ("hello@example.com", "hello"),
        ("alice", "alice"),
        ("bob", "bob"),
    ];
    for (text, name) in names {
        save(name, &base.clone().text(text).build()?)?;
    }

    // "Random avatars": what three calls to Avatar::random can look like.
    // A random avatar is simply the one for a random seed, so fixed seeds
    // give example images that don't change every time this runs.
    for seed in [1, 2, 3] {
        save(&format!("random-{seed}"), &base.clone().seed(seed).build()?)?;
    }

    // Styles. (The Geometric example is sagar.png from above.)
    let sagar = base.text("sagar");
    save(
        "style-pixel",
        &sagar.clone().style(Pixel::default()).build()?,
    )?;
    save(
        "style-pixel-7",
        &sagar
            .clone()
            .style(Pixel {
                grid: 7,
                density: 0.35,
            })
            .build()?,
    )?;

    // Colors.
    let blue = ColorOptions {
        hues: vec![210.0],
        ..Default::default()
    };
    save("color-blue", &sagar.clone().colors(blue).build()?)?;

    let pastel = ColorOptions {
        saturation: 0.45,
        lightness: (0.7, 0.9),
        ..Default::default()
    };
    save("color-pastel", &sagar.clone().colors(pastel).build()?)?;

    let gray = ColorOptions {
        saturation: 0.0,
        ..Default::default()
    };
    save("color-gray", &sagar.clone().colors(gray).build()?)?;

    let light_shapes = ColorOptions {
        lightness: (0.6, 0.9),
        ..Default::default()
    };
    let on_dark = sagar
        .background(Rgba::from_hex("#1e1e2e")?)
        .colors(light_shapes)
        .build()?;
    save("color-dark", &on_dark)?;

    println!("wrote 14 images to ./assets - commit them");
    Ok(())
}
