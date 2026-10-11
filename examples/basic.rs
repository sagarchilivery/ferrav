//! The basics: one avatar per text, random avatars, and getting a random
//! avatar back from its seed.
//!
//! Run with: cargo run --example basic
//! Then open the files in the `out` folder.

use std::fs;

use ferrav::Avatar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    // 1. Same text -> same avatar.
    //    Run this example twice: out/sagar.svg never changes.
    let avatar = Avatar::new("sagar");
    fs::write("out/sagar.svg", avatar.to_svg())?;

    // 2. Different text -> different avatar.
    fs::write("out/ravi.svg", Avatar::new("ravi").to_svg())?;

    // 3. Random: a new avatar every time you run this example.
    let random = Avatar::random("sagar");
    fs::write("out/sagar-random.svg", random.to_svg())?;

    // Every avatar has a seed: one number that describes it completely.
    // Save it (in a database, a file...) to get the same avatar back later.
    let seed = random.seed();
    println!("random avatar seed: {seed}");

    let restored = Avatar::from_seed(seed);
    assert_eq!(random, restored); // exactly the same avatar
    fs::write("out/sagar-restored.svg", restored.to_svg())?;

    // 4. The same avatar as a PNG image instead of SVG.
    fs::write("out/sagar.png", avatar.to_png())?;

    println!("wrote 5 files to ./out");
    Ok(())
}
