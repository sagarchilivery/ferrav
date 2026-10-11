# ferrav

[![Crates.io](https://img.shields.io/crates/v/ferrav.svg)](https://crates.io/crates/ferrav)
[![Documentation](https://docs.rs/ferrav/badge.svg)](https://docs.rs/ferrav)
[![License](https://img.shields.io/crates/l/ferrav.svg)](#license)
[![CI](https://github.com/sagarchilivery/ferrav/actions/workflows/ci.yml/badge.svg)](https://github.com/sagarchilivery/ferrav/actions)

Generate geometric identicons and avatars from any text, in pure Rust.

Export avatars as **SVG** or **PNG**, deterministically or at random, with customizable styles and a tiny dependency footprint.

## Examples

| `sagar` | `ferrav` | `hello@example.com` |
| :---: | :---: | :---: |
| <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/sagar.png" width="96" alt="avatar for sagar"> | <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/ferrav.png" width="96" alt="avatar for ferrav"> | <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/hello.png" width="96" alt="avatar for hello@example.com"> |

## Features

- **Deterministic generation:** the same text always produces the same avatar.
- **Random avatars:** a new avatar on every call, when you don't want one fixed image per text.
- **Seed-based reproducibility:** every avatar has a seed. Store it and recreate the exact same image later.
- **SVG and PNG output:** SVG always; PNG through the default `png` feature.
- **Customizable:** size, padding, background, allowed hues, saturation, lightness, and drawing style.
- **Extensible:** add your own look by implementing the `Style` trait.
- **Lightweight:** zero dependencies for SVG-only builds; PNG adds one small pure-Rust compression crate.

## Installation

```toml
[dependencies]
ferrav = "0.1"
```

For SVG only, with zero dependencies:

```toml
[dependencies]
ferrav = { version = "0.1", default-features = false }
```

## Quick start

### Deterministic avatar

The same text always creates the same avatar:

```rust
use ferrav::Avatar;

let svg: String = Avatar::new("sagar").to_svg();
```

### Random avatar

A different avatar on every call. Save the seed to get it back later:

```rust
use ferrav::Avatar;

let avatar = Avatar::random("sagar");
let seed = avatar.seed(); // store this, e.g. in your users table

// Later: the exact same avatar.
let same_avatar = Avatar::from_seed(seed);
```

### PNG

PNG support is on by default:

```rust
use ferrav::Avatar;

let png: Vec<u8> = Avatar::new("sagar").to_png();
```

## Customization

Every setting goes through the builder, and each one has a sensible default:

```rust
use ferrav::{Avatar, ColorOptions, Mode, Pixel, Rgba};

let avatar = Avatar::builder()
    .text("sagar")
    .mode(Mode::Random)       // or Mode::Deterministic (the default)
    .size(256)                // 16 to 4096 pixels
    .padding(0.1)             // fraction of the size
    .background(Rgba::WHITE)  // default: transparent
    .colors(ColorOptions {
        hues: vec![200.0, 260.0], // only these hues, in degrees
        ..Default::default()
    })
    .style(Pixel::default())  // or Geometric (the default), or your own
    .build()?;
```

### Rebuilding a custom avatar from its seed

A seed only reproduces an avatar together with the settings that made it. Keep one configured builder and reuse it:

```rust
use ferrav::{Avatar, ColorOptions, Mode, Pixel, Rgba};

let settings = Avatar::builder()
    .size(256)
    .padding(0.1)
    .background(Rgba::WHITE)
    .colors(ColorOptions { hues: vec![200.0, 260.0], ..Default::default() })
    .style(Pixel::default());

// A random avatar for a new user.
let avatar = settings.clone().text("sagar").mode(Mode::Random).build()?;
let saved_seed = avatar.seed();

// Later: same settings + saved seed = the exact same avatar.
let same = settings.seed(saved_seed).build()?;
assert_eq!(avatar, same);
```

## Creating your own style

Implement the `Style` trait. Draw inside a `0.0..1.0` square, take every random choice from `rng`, and pick colors from `palette`. ferrav handles scaling, padding and output:

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

Both modes, seeds, sizes, SVG and PNG all work with your style automatically.

## Cargo features

| Feature | Default | Description |
| --- | --- | --- |
| `png` | Yes | PNG output (`Avatar::to_png`, `encode_png`) using `miniz_oxide` |

## Use cases

- User profile avatars and placeholder images
- Anonymous or generated identities
- Developer tools and local-first apps
- Games and creative projects

## Inspiration

ferrav builds on the classic identicon idea: a unique, recognizable picture generated from a piece of text, like the default avatars on GitHub and Gravatar. It brings that idea to Rust with deterministic and random modes, seed-based regeneration, pluggable styles, and SVG and PNG output.

## Stability

The same seed and settings produce the same image within a major version. Changing what an existing seed draws is treated as a breaking change.

## Minimum supported Rust version

Rust **1.85** or newer.

## Documentation

Full API documentation: <https://docs.rs/ferrav>

## Contributing

Contributions are welcome! Before opening a pull request, run the same checks as CI:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.