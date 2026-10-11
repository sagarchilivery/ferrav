//! The two built-in drawing styles: Geometric (the default) and Pixel.
//!
//! Run with: cargo run --example styles

use std::fs;

use ferrav::{Avatar, Geometric, Pixel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // Geometric is the default style, so these two are the same avatar.
    let default = Avatar::new("sagar");
    let geometric = Avatar::builder().text("sagar").style(Geometric).build()?;
    assert_eq!(default, geometric);
    fs::write("out/style-geometric.svg", geometric.to_svg())?;

    // Pixel: GitHub-style squares, mirrored left to right like a face.
    let pixel = Avatar::builder()
        .text("sagar")
        .style(Pixel::default())
        .build()?;
    fs::write("out/style-pixel.svg", pixel.to_svg())?;

    // Pixel has two settings:
    // - grid:    squares per row and column (2 to 64, default 5)
    // - density: chance that each square is filled (0.0 to 1.0, default 0.5)
    for (grid, density) in [(5, 0.3), (7, 0.5), (9, 0.7), (16, 0.5)] {
        let avatar = Avatar::builder()
            .text("sagar")
            .style(Pixel { grid, density })
            .build()?;
        let file = format!("out/style-pixel-{grid}x{grid}-{density}.svg");
        fs::write(file, avatar.to_svg())?;
    }

    println!("wrote 6 files to ./out");
    Ok(())
}
