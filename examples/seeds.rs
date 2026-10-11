//! Seeds: give each user a random avatar once, then show the same one forever.
//! This is the pattern most apps want.
//!
//! Run with: cargo run --example seeds

use ferrav::{Avatar, Mode, Pixel, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- With the default settings ---
    // Sign-up: make a random avatar and save its seed with the user.
    let avatar = Avatar::random("sagar");
    let saved_seed: u64 = avatar.seed();
    println!("saved seed: {saved_seed}");

    // Later (another request, another day): rebuild it from the seed.
    let again = Avatar::from_seed(saved_seed);
    assert_eq!(avatar, again);

    // --- With custom settings ---
    // A seed only stands for the random choices. Size, colors and style are
    // separate, so rebuild with the same settings as before. The easiest way
    // is to keep one configured builder and reuse it.
    let settings = Avatar::builder()
        .size(96)
        .background(Rgba::WHITE)
        .style(Pixel::default());

    let avatar = settings.clone().text("sagar").mode(Mode::Random).build()?;
    let saved_seed = avatar.seed();

    let again = settings.clone().seed(saved_seed).build()?;
    assert_eq!(avatar, again); // same settings + same seed = same avatar

    // The same seed with *different* settings gives a different picture:
    let different = Avatar::from_seed(saved_seed); // default settings, not Pixel
    assert_ne!(avatar, different);

    // --- Choosing the mode at runtime ---
    // Handy when the mode comes from a config file or a checkbox.
    let wants_random = false;
    let mode = if wants_random {
        Mode::Random
    } else {
        Mode::Deterministic
    };
    let avatar = settings.text("sagar").mode(mode).build()?;

    // A deterministic seed only depends on the text: same on every run and machine.
    println!("deterministic seed for \"sagar\": {}", avatar.seed());
    assert_eq!(avatar.seed(), Avatar::new("sagar").seed());

    println!("all seed checks passed");
    Ok(())
}
