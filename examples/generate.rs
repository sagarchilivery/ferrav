//! Writes a few avatars into ./out so you can open them.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example generate
//! ```

use std::fs;

use ferrav::{Avatar, ColorOptions, Mode, Pixel, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // 1. Deterministic SVG.
    //
    // Same input always produces the same avatar.
    // Great for user profiles, seeded identities, etc.
    let deterministic = Avatar::new("sagar");
    fs::write("out/sagar.svg", deterministic.to_svg())?;

    // 2. Random avatar with reproducible seed.
    //
    // Save the seed if you want to generate the same avatar later.
    let random = Avatar::random("sagar");
    println!("random avatar seed: {}", random.seed());
    fs::write("out/sagar-random.svg", random.to_svg())?;

    // 3. Fully customized avatar.
    //
    // Uses:
    // - deterministic generation
    // - custom canvas size
    // - custom padding
    // - custom background
    // - custom palette
    // - pixel style
    let custom = Avatar::builder()
        .text("sagar")
        .mode(Mode::Deterministic)
        .size(256)
        .padding(0.1)
        .background(Rgba::from_hex("#f4f4f5")?)
        .colors(ColorOptions {
            hues: vec![200.0, 260.0],
            saturation: 0.7,
            ..Default::default()
        })
        .style(Pixel::default())
        .build()?;
    fs::write("out/sagar-pixel.png", custom.to_png())?;

    // 4. Generate multiple avatars.
    //
    // Demonstrates typical application usage.
    for name in ["alice", "bob", "charlie"] {
        fs::write(format!("out/{name}.svg"), Avatar::new(name).to_svg())?;
    }

    println!("generated avatars in ./out");

    Ok(())
}