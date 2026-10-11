# ferrav

[![Crates.io](https://img.shields.io/crates/v/ferrav.svg)](https://crates.io/crates/ferrav)
[![Docs.rs](https://docs.rs/ferrav/badge.svg)](https://docs.rs/ferrav)
[![License](https://img.shields.io/crates/l/ferrav.svg)](#license)
[![CI](https://github.com/sagarchilivery/ferrav/actions/workflows/ci.yml/badge.svg)](https://github.com/sagarchilivery/ferrav/actions/workflows/ci.yml)

**Turn any text into a unique little picture.** Give ferrav a username, an email or an ID, and it draws an avatar for it, as an **SVG** or a **PNG**. Same text, same picture, every time. Written in pure Rust.

<p align="center">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/sagar.png" width="96" alt="avatar for sagar">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/ferrav.png" width="96" alt="avatar for ferrav">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/hello.png" width="96" alt="avatar for hello@example.com">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/alice.png" width="96" alt="avatar for alice">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/bob.png" width="96" alt="avatar for bob">
</p>
<p align="center"><sub><code>sagar</code> · <code>ferrav</code> · <code>hello@example.com</code> · <code>alice</code> · <code>bob</code></sub></p>

## Contents

- [What can it do?](#what-can-it-do)
- [Quick start](#quick-start)
- [The two modes: same or random](#the-two-modes-same-or-random)
- [Recipes](#recipes): styles, colors, size, saving seeds, web pages, custom styles, errors
- [All settings at a glance](#all-settings-at-a-glance)
- [Runnable examples](#runnable-examples)
- [FAQ](#faq)

## What can it do?

- 🎯 **Same text, same avatar.** Perfect for profile pictures: no need to store an image.
- 🎲 **Random avatars**, when you want a fresh one every time.
- 🌱 **Seeds:** every avatar has a number that brings it back exactly.
- 🖼 **SVG and PNG** output, plus raw pixels if you need them.
- 🎨 **Customizable:** two built-in styles, colors, size, padding and background.
- 🧩 **Pluggable:** write your own style with one function.
- ⚡ **Tiny:** zero dependencies if you only need SVG.

## Quick start

**1. Create a project and add ferrav** (skip `cargo new` if you already have one):

```bash
cargo new my-avatars
cd my-avatars
cargo add ferrav
```

**2. Replace `src/main.rs` with:**

```rust
use ferrav::Avatar;

fn main() {
    let avatar = Avatar::new("sagar");

    std::fs::write("sagar.svg", avatar.to_svg()).unwrap();
    std::fs::write("sagar.png", avatar.to_png()).unwrap();

    println!("Created sagar.svg and sagar.png");
}
```

**3. Run it:**

```bash
cargo run
```

Open `sagar.svg` or `sagar.png` and you'll see your first avatar. Run it again and you get exactly the same picture. Change `"sagar"` to your own name and you get a different one.

> **SVG or PNG?** SVG is text that browsers draw directly. It stays sharp at any size and is tiny, so it's best for websites. PNG is a normal image file that works everywhere else: apps, chat messages, image editors.

## The two modes: same or random

| | **Deterministic** (the default) | **Random** |
| --- | --- | --- |
| Code | `Avatar::new("sagar")` | `Avatar::random("sagar")` |
| Same text gives... | the same avatar, every time, on every computer | a new avatar on every call |
| Good for | profile pictures without storing anything | letting users "roll" a new look |

Here are three calls to `Avatar::random("sagar")`:

<p>
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/random-1.png" width="80" alt="random avatar 1">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/random-2.png" width="80" alt="random avatar 2">
  <img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/random-3.png" width="80" alt="random avatar 3">
</p>

**How do I keep a random avatar?** Every avatar has a **seed**: one number that describes it completely. Save the seed, and you can rebuild the exact same avatar later:

```rust
use ferrav::Avatar;

let avatar = Avatar::random("sagar");
let seed = avatar.seed(); // a u64 number: save it, e.g. in your users table

// Later, even on another day or another server:
let same = Avatar::from_seed(seed);
assert_eq!(avatar, same);
```

## Recipes

Most recipes below use `?` to handle errors. Put them inside a `main` like this one:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // recipe code goes here

    Ok(())
}
```

Every setting goes through **the builder**: start with `Avatar::builder()`, add only the settings you want, and finish with `.build()?`. Anything you don't set keeps a sensible default.

### Pick a style

<table>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/sagar.png" width="80" alt="Geometric style"><br><code>Geometric</code> (default)</td>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/style-pixel.png" width="80" alt="Pixel style"><br><code>Pixel::default()</code></td>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/style-pixel-7.png" width="80" alt="Pixel style, 7 by 7 grid"><br><code>Pixel { grid: 7, density: 0.35 }</code></td>
  </tr>
</table>

```rust
use ferrav::{Avatar, Pixel};

// GitHub-style squares, mirrored left to right like a face.
let avatar = Avatar::builder()
    .text("sagar")
    .style(Pixel::default())
    .build()?;

// More, smaller squares, with fewer of them filled in.
let avatar = Avatar::builder()
    .text("sagar")
    .style(Pixel { grid: 7, density: 0.35 })
    .build()?;
```

`grid` is the number of squares per row (2 to 64). `density` is the chance that each square is filled, from `0.0` (almost empty) to `1.0` (solid).

### Change the colors

<table>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/color-blue.png" width="80" alt="only blue"><br>only blue</td>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/color-pastel.png" width="80" alt="pastel"><br>pastel</td>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/color-gray.png" width="80" alt="grayscale"><br>grayscale</td>
    <td align="center"><img src="https://raw.githubusercontent.com/sagarchilivery/ferrav/main/assets/color-dark.png" width="80" alt="dark background"><br>dark background</td>
  </tr>
</table>

Colors are described with three simple numbers:

- **Hue** is *which* color, as an angle on the color wheel: `0` red, `60` yellow, `120` green, `180` cyan, `240` blue, `300` magenta.
- **Saturation** is *how colorful*: `0.0` is gray, `1.0` is as vivid as possible.
- **Lightness** is *how bright*, given as `(darkest, lightest)`: `0.0` is black, `1.0` is white.

```rust
use ferrav::{Avatar, ColorOptions};

// Only blues. List several hues to allow several colors.
let avatar = Avatar::builder()
    .text("sagar")
    .colors(ColorOptions {
        hues: vec![210.0],
        ..Default::default() // keep the defaults for everything else
    })
    .build()?;

// Soft pastel colors: not very colorful, quite light.
let avatar = Avatar::builder()
    .text("sagar")
    .colors(ColorOptions {
        saturation: 0.45,
        lightness: (0.7, 0.9),
        ..Default::default()
    })
    .build()?;
```

`..Default::default()` means "every field I didn't write keeps its default value". You'll see it a lot in Rust.

### Size, padding and background

```rust
use ferrav::{Avatar, Rgba};

let avatar = Avatar::builder()
    .text("sagar")
    .size(256)                         // width and height in pixels (16 to 4096)
    .padding(0.1)                      // 10% empty space on each side
    .background(Rgba::WHITE)           // default: transparent
    .build()?;

// Any background color works:
let dark = Rgba::from_hex("#1e1e2e")?; // a hex code, like in CSS
let cream = Rgba::rgb(255, 248, 231);  // or red, green, blue from 0 to 255
```

### Give each user a random avatar that never changes

This is what most apps want: pick a random avatar when a user signs up, then show that same avatar forever.

A seed only stands for the *random choices*. Size, colors and style are separate, so **rebuild with the same settings** you used the first time. The easiest way is to configure one builder and reuse it:

```rust
use ferrav::{Avatar, Mode, Pixel, Rgba};

// Your app's avatar settings, written once.
let settings = Avatar::builder()
    .size(96)
    .background(Rgba::WHITE)
    .style(Pixel::default());

// When a user signs up:
let avatar = settings.clone().text("sagar").mode(Mode::Random).build()?;
let seed = avatar.seed(); // save this with the user

// Every time you show their avatar:
let same = settings.clone().seed(seed).build()?;
assert_eq!(avatar, same);
```

`.clone()` makes a copy of the settings, so the original `settings` can be used again for the next user.

### Show it on a web page

An SVG is just text, so you can put it straight into your HTML:

```rust
use ferrav::Avatar;

let svg = Avatar::new("sagar").to_svg();
let html = format!("<div class=\"avatar\">{svg}</div>");
```

If your web server sends avatars as files, use these content types:

- SVG: `Content-Type: image/svg+xml`
- PNG: `Content-Type: image/png`

Deterministic avatars never change for the same text and settings, so browsers can cache them for a long time.

### Make your own style

A style is anything that implements the `Style` trait: one function that returns a list of shapes. Here's one that draws rings:

```rust
use ferrav::{Avatar, Palette, Point, Rng, Shape, Style};

#[derive(Debug)]
struct Rings {
    count: u32,
}

impl Style for Rings {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let colors = palette.all();
        (0..self.count)
            .map(|i| {
                // Biggest circle first, so the smaller ones are drawn on top.
                let radius = 0.5 * (self.count - i) as f32 / self.count as f32;
                Shape::circle(Point::new(0.5, 0.5), radius, *rng.pick(&colors))
            })
            .collect()
    }
}

let avatar = Avatar::builder()
    .text("sagar")
    .style(Rings { count: 4 })
    .build()?;
```

Three rules for a style:

1. **Draw inside a square from `(0, 0)` (top-left) to `(1, 1)` (bottom-right).** ferrav scales your drawing to the real size and adds the padding.
2. **Take every random choice from `rng`.** That's what makes seeds and deterministic mode work.
3. **Pick colors from `palette`** (`dark`, `mid`, `light`, `dark_gray`, `light_gray`), so the user's color settings apply.

Follow them, and your style automatically works with both modes, seeds, every size, and SVG and PNG output.

### Handle errors

Settings that are out of range give an error with a readable message. They never crash your program:

```rust
use ferrav::Avatar;

match Avatar::builder().size(5).build() {
    Ok(avatar) => println!("made a {} px avatar", avatar.size()),
    Err(error) => println!("could not make it: {error}"),
}
// prints: could not make it: size 5 is out of range (allowed: 16 to 4096 pixels)
```

`Avatar::new`, `Avatar::random` and `Avatar::from_seed` always use valid default settings, so they can't fail and don't need `?`.

## All settings at a glance

**Builder settings:**

| Method | What it does | Default | Allowed values |
| --- | --- | --- | --- |
| `.text("...")` | the text to make an avatar for | `""` | any text |
| `.mode(...)` | same avatar or a random one | `Mode::Deterministic` | `Mode::Deterministic`, `Mode::Random` |
| `.seed(n)` | rebuild a saved avatar (ignores text and mode) | none | any `u64` |
| `.size(px)` | width and height in pixels | `128` | `16` to `4096` |
| `.padding(f)` | empty space around the drawing | `0.08` | `0.0` up to (not including) `0.5` |
| `.background(c)` | background color | transparent | any `Rgba` |
| `.colors(...)` | which colors may be used | see below | see below |
| `.style(...)` | the drawing style | `Geometric` | `Geometric`, `Pixel { .. }`, or your own |

**`ColorOptions`:**

| Field | What it does | Default |
| --- | --- | --- |
| `hues` | allowed hues in degrees (empty = any) | `vec![]` |
| `saturation` | how colorful the colored shapes are (0.0 to 1.0) | `0.5` |
| `lightness` | `(darkest, lightest)` for the colored shapes | `(0.4, 0.8)` |
| `grayscale_saturation` | how tinted the gray shapes are (0.0 = pure gray) | `0.0` |
| `grayscale_lightness` | `(darkest, lightest)` for the gray shapes | `(0.3, 0.9)` |

**What you can get from an `Avatar`:**

| Method | Returns |
| --- | --- |
| `.to_svg()` | the SVG as a `String` |
| `.to_png()` | the PNG file as bytes (`Vec<u8>`) |
| `.to_rgba()` | raw pixels: 4 bytes per pixel (red, green, blue, alpha) |
| `.seed()` | the seed (`u64`) to rebuild it later |
| `.size()` | the width and height in pixels |
| `.scene()` | the shapes it's made of, if you want to draw them yourself |

The full API documentation is on [docs.rs/ferrav](https://docs.rs/ferrav).

## Runnable examples

The [`examples`](https://github.com/sagarchilivery/ferrav/tree/main/examples) folder has small, commented programs for every feature. To run them, download the repository first:

```bash
git clone https://github.com/sagarchilivery/ferrav
cd ferrav
cargo run --example gallery
```

| Example | What it shows |
| --- | --- |
| `gallery` | **Start here.** Every feature on one page: open `out/gallery.html` |
| `basic` | Same-text avatars, random avatars, seeds, SVG and PNG |
| `styles` | `Geometric` and `Pixel` with different grids and densities |
| `colors` | Hues, saturation, lightness, backgrounds, size and padding |
| `seeds` | Giving each user a random avatar that never changes |
| `custom_style` | Writing your own styles (rings and stripes) |
| `advanced` | Errors, looking inside an avatar, raw pixels, drawing by hand |

Each one writes its files into an `out` folder.

## FAQ

**I saved a seed, but `Avatar::from_seed` gives a different picture.**
`from_seed` uses the default settings. If the original avatar used a different size, colors or style, rebuild it with the same settings plus `.seed(...)`. See [Give each user a random avatar that never changes](#give-each-user-a-random-avatar-that-never-changes).

**Will my avatars change when I update ferrav?**
Not within compatible versions. The same text, seed and settings always draw the same avatar in every `0.1.x` release (and in every `1.x` release after 1.0). Changing what an existing seed draws counts as a breaking change and only happens in a new `0.x` (or major) version.

**Does the avatar reveal the text it was made from?**
The picture doesn't contain the text. But anyone who knows or guesses the text can draw the same avatar. Treat avatars as decoration, not as a way to hide or verify someone's identity.

**Can I use it without PNG support?**
Yes. Turn off the default `png` feature and ferrav has zero dependencies:

```toml
[dependencies]
ferrav = { version = "0.1", default-features = false }
```

You still get SVG output and raw pixels.

## Cargo features

| Feature | On by default | What it adds |
| --- | --- | --- |
| `png` | ✅ | PNG output (`Avatar::to_png`, `encode_png`), using the small pure-Rust `miniz_oxide` crate |

## Inspiration

Inspired by GitHub identicons and Gravatar, `ferrav` brings the same idea to Rust with deterministic generation, random mode, reproducible seeds, SVG and PNG output, and a fully extensible styling system.

## Minimum supported Rust version

Rust **1.85** or newer.

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