# ferrav

[![Crates.io](https://img.shields.io/crates/v/ferrav.svg)](https://crates.io/crates/ferrav)
[![Docs.rs](https://docs.rs/ferrav/badge.svg)](https://docs.rs/ferrav)
[![License](https://img.shields.io/crates/l/ferrav.svg)](#license)
[![CI](https://github.com/sagarchilivery/ferrav/actions/workflows/ci.yml/badge.svg)](https://github.com/sagarchilivery/ferrav/actions/workflows/ci.yml)

Generate beautiful geometric avatars from any text.

`ferrav` is a lightweight, pure Rust avatar generator that produces deterministic or random identicons as **SVG** or **PNG**. It has zero dependencies for SVG-only builds, customizable colors and styles, and lets you recreate avatars exactly from saved seeds.

## Example

| `sagar` | `ferrav` | `hello@example.com` |
| :---: | :---: | :---: |
| <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/sagar.png" width="96" alt="avatar for sagar"> | <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/ferrav.png" width="96" alt="avatar for ferrav"> | <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/hello.png" width="96" alt="avatar for hello@example.com"> |

## Features

- 🎯 Deterministic avatars (same text → same image)
- 🎲 Random avatar generation
- 🌱 Seed-based reproducibility
- 🖼 SVG output
- 📦 PNG output (enabled by default)
- 🎨 Custom colors, padding, size, background, and styles
- 🧩 Pluggable `Style` trait for custom generators
- ⚡ Zero dependencies for SVG-only builds

## Installation

```toml
[dependencies]
ferrav = "0.1"
```

SVG only, with zero dependencies:

```toml
[dependencies]
ferrav = { version = "0.1", default-features = false }
```

## Quick start

### Deterministic avatar

The same text always produces the same avatar:

```rust
use ferrav::Avatar;

let svg = Avatar::new("sagar").to_svg();
```

### Random avatar

A new avatar on every call. Save the seed to get it back later:

```rust
use ferrav::Avatar;

let avatar = Avatar::random("sagar");
let seed = avatar.seed(); // store this, e.g. in your users table

// Later: the exact same avatar.
let same = Avatar::from_seed(seed);
assert_eq!(avatar, same);
```

### PNG

```rust
use ferrav::Avatar;

let png: Vec<u8> = Avatar::new("sagar").to_png();
```

## Customization

```rust
use ferrav::{Avatar, ColorOptions, Mode, Pixel, Rgba};

let avatar = Avatar::builder()
    .text("sagar")
    .mode(Mode::Random)       // or Mode::Deterministic (the default)
    .size(256)                // 16 to 4096 pixels
    .padding(0.1)             // fraction of the size, below 0.5
    .background(Rgba::WHITE)  // default: transparent
    .colors(ColorOptions {
        hues: vec![200.0, 260.0], // only these hues, in degrees
        ..Default::default()
    })
    .style(Pixel::default())  // or Geometric (the default), or your own
    .build()?;
```

Every option has a sensible default, so you only need to configure what you care about.

### Recreating avatars from saved seeds

A seed reproduces an avatar only together with the settings that made it. Keep one configured builder and reuse it:

```rust
use ferrav::{Avatar, Mode, Pixel};

let settings = Avatar::builder()
    .size(256)
    .padding(0.1)
    .style(Pixel::default());

// A random avatar for a new user.
let avatar = settings.clone().text("sagar").mode(Mode::Random).build()?;
let seed = avatar.seed();

// Later: same settings + saved seed = the exact same avatar.
let same = settings.seed(seed).build()?;
assert_eq!(avatar, same);
```

## Creating custom styles

Implement the `Style` trait. Draw inside a `0.0..1.0` square, take every random choice from `rng`, and pick colors from `palette`:

```rust
use ferrav::{Avatar, Palette, Point, Rng, Shape, Style};

#[derive(Debug)]
struct Dot;

impl Style for Dot {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let radius = 0.2 + rng.next_f32() * 0.3;
        vec![Shape::circle(Point::new(0.5, 0.5), radius, palette.mid)]
    }
}

let avatar = Avatar::builder().text("sagar").style(Dot).build()?;
```

Your custom style automatically supports deterministic and random modes, seeds, custom sizes and padding, and both SVG and PNG output.

## Built-in styles

| Style | Description |
| --- | --- |
| `Geometric` | Symmetric geometric patterns (the default) |
| `Pixel` | GitHub-style pixel art, with adjustable `grid` and `density` |

## Cargo features

| Feature | Default | Description |
| --- | --- | --- |
| `png` | ✅ | PNG output (`Avatar::to_png`, `encode_png`) using `miniz_oxide` |

## Use cases

- User profile avatars
- Placeholder images
- Anonymous identities
- Developer tools
- Games
- Local-first applications

## Inspiration

Inspired by GitHub identicons and Gravatar, `ferrav` brings the same idea to Rust with deterministic generation, random mode, reproducible seeds, SVG and PNG output, and a fully extensible styling system.

## Stability

The same seed and settings always produce the same avatar within compatible versions: any `0.1.x` release today, and any `1.x.y` release after 1.0. Changing what an existing seed draws is treated as a breaking change and gets a new `0.x` (or major) version.

## Minimum supported Rust version

Rust **1.85** or newer.

## Documentation

Full API documentation: <https://docs.rs/ferrav>

## Contributing

Pull requests and issues are always welcome. Before opening a pull request, run the same checks as CI:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.