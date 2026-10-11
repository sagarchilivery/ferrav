//! Make your own style. Implement one method, and your style gets sizes,
//! padding, colors, both modes, seeds, SVG and PNG for free.
//!
//! Run with: cargo run --example custom_style

use std::fs;

use ferrav::{Avatar, Mode, Palette, Point, Rng, Shape, Style};

/// Circles inside each other, biggest first, so the smaller ones are drawn on top.
#[derive(Debug)]
struct Rings {
    count: u32,
}

impl Style for Rings {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let colors = palette.all();
        (0..self.count)
            .map(|i| {
                let radius = 0.5 * (self.count - i) as f32 / self.count as f32;
                Shape::circle(Point::new(0.5, 0.5), radius, *rng.pick(&colors))
            })
            .collect()
    }
}

/// Horizontal stripes in the palette's colors.
#[derive(Debug)]
struct Stripes;

impl Style for Stripes {
    fn draw(&self, rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        let colors = [palette.dark, palette.mid, palette.light];
        let count = 3 + rng.below(4); // 3 to 6 stripes
        let height = 1.0 / count as f32;

        (0..count)
            .map(|i| {
                let top = i as f32 * height;
                let corners = vec![
                    Point::new(0.0, top),
                    Point::new(1.0, top),
                    Point::new(1.0, top + height),
                    Point::new(0.0, top + height),
                ];
                Shape::polygon(corners, *rng.pick(&colors))
            })
            .collect()
    }
}

// The rules for a style:
// 1. Draw inside a square from (0, 0) at the top-left to (1, 1) at the
//    bottom-right. ferrav scales it to the real size and adds the padding.
// 2. Take every random choice from `rng`. That's what makes seeds work.
// 3. Pick colors from `palette`, so the user's color settings apply.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("out")?;

    let rings = Avatar::builder()
        .text("sagar")
        .style(Rings { count: 4 })
        .build()?;
    fs::write("out/custom-rings.svg", rings.to_svg())?;

    let stripes = Avatar::builder().text("sagar").style(Stripes).build()?;
    fs::write("out/custom-stripes.svg", stripes.to_svg())?;

    // Everything else works with your style too, like random mode and seeds.
    let settings = Avatar::builder().style(Stripes);
    let random = settings.clone().text("sagar").mode(Mode::Random).build()?;
    let restored = settings.seed(random.seed()).build()?;
    assert_eq!(random, restored);
    fs::write("out/custom-stripes-random.svg", random.to_svg())?;

    println!("wrote 3 files to ./out");
    Ok(())
}
