//! Colors, backgrounds, size and padding.
//!
//! Run with: cargo run --example colors

use std::fs;

use ferrav::{Avatar, ColorOptions, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // Hue = which color, as an angle on the color wheel:
    // 0 = red, 60 = yellow, 120 = green, 180 = cyan, 240 = blue, 300 = magenta.
    // By default any hue can be picked; `hues` limits the choice.
    let blue = ColorOptions {
        hues: vec![210.0],
        ..Default::default()
    };
    let warm = ColorOptions {
        hues: vec![0.0, 30.0, 45.0],
        ..Default::default()
    };

    // Saturation = how colorful: 0.0 is gray, 1.0 is as vivid as possible.
    // Lightness = how bright, as (darkest, lightest): 0.0 is black, 1.0 is white.
    let vivid = ColorOptions {
        saturation: 0.9,
        lightness: (0.35, 0.65),
        ..Default::default()
    };
    let pastel = ColorOptions {
        saturation: 0.45,
        lightness: (0.7, 0.9),
        ..Default::default()
    };
    let gray = ColorOptions {
        saturation: 0.0,
        ..Default::default()
    };

    let palettes = [
        ("blue", blue),
        ("warm", warm),
        ("vivid", vivid),
        ("pastel", pastel),
        ("gray", gray),
    ];
    for (name, colors) in palettes {
        let avatar = Avatar::builder().text("sagar").colors(colors).build()?;
        fs::write(format!("out/color-{name}.svg"), avatar.to_svg())?;
    }

    // Background: transparent by default.
    let on_white = Avatar::builder()
        .text("sagar")
        .background(Rgba::WHITE)
        .build()?;
    fs::write("out/background-white.svg", on_white.to_svg())?;

    // Any color from a hex code, like in CSS...
    let on_dark = Avatar::builder()
        .text("sagar")
        .background(Rgba::from_hex("#1e1e2e")?)
        // Lighter shapes so they stand out on a dark background.
        .colors(ColorOptions {
            lightness: (0.6, 0.9),
            ..Default::default()
        })
        .build()?;
    fs::write("out/background-dark.svg", on_dark.to_svg())?;

    // ...or from red, green and blue values (0 to 255).
    let on_cream = Avatar::builder()
        .text("sagar")
        .background(Rgba::rgb(255, 248, 231))
        .build()?;
    fs::write("out/background-cream.svg", on_cream.to_svg())?;

    // Size: width and height in pixels (16 to 4096, default 128).
    // Padding: empty space around the drawing, as a fraction of the size
    // (0.0 up to, not including, 0.5; default 0.08). 0.2 = 20% on each side.
    for (size, padding) in [(64, 0.0), (128, 0.08), (256, 0.2)] {
        let avatar = Avatar::builder()
            .text("sagar")
            .size(size)
            .padding(padding)
            .background(Rgba::WHITE)
            .build()?;
        let file = format!("out/size-{size}-padding-{padding}.svg");
        fs::write(file, avatar.to_svg())?;
    }

    println!("wrote 11 files to ./out");
    Ok(())
}
