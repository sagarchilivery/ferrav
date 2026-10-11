//! Writes the avatar images used in README.md into ./assets.
//!
//! Run with: cargo run --example readme_images

use std::fs;

use ferrav::Avatar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("assets")?;

    for (text, file) in [
        ("sagar", "sagar"),
        ("ferrav", "ferrav"),
        ("hello@example.com", "hello"),
    ] {
        let avatar = Avatar::builder().text(text).size(192).build()?;
        fs::write(format!("assets/{file}.png"), avatar.to_png())?;
    }

    println!("wrote 3 images to ./assets - commit them");
    Ok(())
}