//! Tests that use ferrav exactly the way other people's code will.

use ferrav::{Avatar, ColorOptions, Error, Mode, Palette, Pixel, Point, Rng, Shape, Style};

#[test]
fn same_text_gives_same_avatar() {
    assert_eq!(Avatar::new("sagar"), Avatar::new("sagar"));
}

#[test]
fn different_text_gives_different_avatar() {
    assert_ne!(Avatar::new("sagar").to_svg(), Avatar::new("ravi").to_svg());
}

#[test]
fn random_mode_gives_a_new_avatar_each_call() {
    let first = Avatar::random("sagar");
    let second = Avatar::random("sagar");
    assert_ne!(first.seed(), second.seed());
}

#[test]
fn a_saved_seed_brings_the_avatar_back() {
    let original = Avatar::random("sagar");
    assert_eq!(Avatar::from_seed(original.seed()), original);
}

#[test]
fn seeds_work_with_custom_settings_too() {
    let settings = Avatar::builder()
        .size(64)
        .mode(Mode::Random)
        .style(Pixel::default());
    let original = settings.clone().text("sagar").build().unwrap();
    let again = settings.seed(original.seed()).build().unwrap();
    assert_eq!(original, again);
}

#[test]
fn builder_defaults_match_avatar_new() {
    let from_new = Avatar::new("sagar");
    let from_builder = Avatar::builder().text("sagar").build().unwrap();

    assert_eq!(from_new, from_builder);
}

#[test]
fn restored_seed_renders_identically() {
    let original = Avatar::random("sagar");
    let restored = Avatar::from_seed(original.seed());

    assert_eq!(original.to_svg(), restored.to_svg());
    assert_eq!(original.to_rgba(), restored.to_rgba());
}

#[test]
fn bad_settings_are_rejected() {
    assert!(matches!(
        Avatar::builder().size(4).build(),
        Err(Error::InvalidSize { .. })
    ));
    assert!(matches!(
        Avatar::builder().padding(0.6).build(),
        Err(Error::InvalidPadding(_))
    ));
    let colors = ColorOptions {
        saturation: 2.0,
        ..Default::default()
    };
    assert!(matches!(
        Avatar::builder().colors(colors).build(),
        Err(Error::InvalidColor(_))
    ));
}

#[test]
fn invalid_padding_values_are_rejected() {
    // 0.5 is the first value that is NOT allowed; NaN must never slip through.
    for padding in [-0.1, 0.5, 0.51, 1.0, f32::NAN] {
        assert!(
            matches!(
                Avatar::builder().padding(padding).build(),
                Err(Error::InvalidPadding(_))
            ),
            "padding {padding} should be rejected"
        );
    }
}

#[test]
fn svg_is_well_formed() {
    let svg = Avatar::builder().size(64).build().unwrap().to_svg();

    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains(r#"width="64""#));
    assert!(svg.contains(r#"height="64""#));
    assert!(svg.contains(r#"viewBox="0 0 64 64""#));
}

#[test]
fn rgba_has_expected_length() {
    let avatar = Avatar::builder().size(64).build().unwrap();

    assert_eq!(avatar.to_rgba().len(), 64 * 64 * 4);
}

#[test]
fn empty_text_is_deterministic() {
    assert_eq!(Avatar::new(""), Avatar::new(""));
}

#[test]
fn unicode_text_is_deterministic() {
    assert_eq!(Avatar::new("नमस्ते 👋"), Avatar::new("नमस्ते 👋"));
}

#[test]
fn long_text_is_deterministic() {
    let text = "a".repeat(10_000);

    assert_eq!(Avatar::new(&text), Avatar::new(&text));
}

#[test]
fn same_seed_draws_the_same_design_at_any_size() {
    let small = Avatar::builder().text("sagar").size(64).build().unwrap();
    let big = Avatar::builder()
        .seed(small.seed())
        .size(256)
        .build()
        .unwrap();

    // Same shapes in the same colors and order; only their positions are scaled.
    let fills = |avatar: &Avatar| {
        avatar
            .scene()
            .shapes
            .iter()
            .map(|shape| shape.fill)
            .collect::<Vec<_>>()
    };
    assert_eq!(fills(&small), fills(&big));
    assert_eq!((small.size(), big.size()), (64, 256));
}

#[test]
fn many_inputs_do_not_panic() {
    for i in 0..10_000 {
        let avatar = Avatar::new(&format!("user{i}"));

        assert!(!avatar.to_svg().is_empty());
    }
}

/// A tiny custom style: one big dot. Proves outside code can plug in a style.
#[derive(Debug)]
struct OneDot;

impl Style for OneDot {
    fn draw(&self, _rng: &mut Rng, palette: &Palette) -> Vec<Shape> {
        vec![Shape::circle(Point::new(0.5, 0.5), 0.5, palette.mid)]
    }
}

#[test]
fn custom_styles_plug_in() {
    let avatar = Avatar::builder().style(OneDot).build().unwrap();
    assert_eq!(avatar.scene().shapes.len(), 1);
    assert!(avatar.to_svg().contains("<circle"));
}

#[cfg(feature = "png")]
#[test]
fn png_output_decodes_to_the_same_pixels() {
    let avatar = Avatar::builder().text("sagar").size(48).build().unwrap();
    let bytes = avatar.to_png();

    // Decode with the independent `png` crate to prove our file is valid.
    let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();

    assert_eq!((info.width, info.height), (48, 48));
    assert_eq!(&pixels[..info.buffer_size()], avatar.to_rgba().as_slice());
}