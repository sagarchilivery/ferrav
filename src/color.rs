//! Colors: the [`Rgba`] type, HSL conversion, color settings and palettes.

use std::fmt;

use crate::error::Error;
use crate::rng::Rng;

/// A color: red, green, blue and alpha (opacity), each from 0 to 255.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    /// Red, 0–255.
    pub r: u8,
    /// Green, 0–255.
    pub g: u8,
    /// Blue, 0–255.
    pub b: u8,
    /// Alpha: 0 = invisible, 255 = solid.
    pub a: u8,
}

impl Rgba {
    /// Solid white.
    pub const WHITE: Rgba = Rgba::rgb(255, 255, 255);

    /// Solid black.
    pub const BLACK: Rgba = Rgba::rgb(0, 0, 0);

    /// Fully transparent.
    pub const TRANSPARENT: Rgba = Rgba::new(0, 0, 0, 0);

    /// A solid color from red, green and blue.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// A color with an explicit alpha (opacity).
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Builds a solid color from HSL.
    ///
    /// Hue is in degrees: `0` = red, `120` = green, `240` = blue. Any hue
    /// works because it wraps around, so `-120` is the same as `240`.
    ///
    /// Saturation and lightness go from `0.0` to `1.0` and are clamped to
    /// that range. If any value is not a finite number (`NaN` or infinity),
    /// this returns [`Rgba::BLACK`]. Use [`Rgba::try_from_hsl`] to get an
    /// error instead.
    pub fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::try_from_hsl(hue, saturation, lightness).unwrap_or(Self::BLACK)
    }

    /// Like [`Rgba::from_hsl`], but returns an error if any value is not a
    /// finite number.
    pub fn try_from_hsl(hue: f32, saturation: f32, lightness: f32) -> Result<Self, Error> {
        if !hue.is_finite() || !saturation.is_finite() || !lightness.is_finite() {
            return Err(Error::InvalidColor(
                format!(
                    "HSL values must be finite numbers, got ({hue}, {saturation}, {lightness})"
                )
                .into(),
            ));
        }

        let saturation = saturation.clamp(0.0, 1.0);
        let lightness = lightness.clamp(0.0, 1.0);

        let h = hue.rem_euclid(360.0) / 60.0;
        let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = chroma * (1.0 - (h % 2.0 - 1.0).abs());

        let (r, g, b) = match h as u32 {
            0 => (chroma, x, 0.0),
            1 => (x, chroma, 0.0),
            2 => (0.0, chroma, x),
            3 => (0.0, x, chroma),
            4 => (x, 0.0, chroma),
            _ => (chroma, 0.0, x),
        };

        let m = lightness - chroma / 2.0;
        let to_byte = |value: f32| ((value + m) * 255.0).round().clamp(0.0, 255.0) as u8;

        Ok(Self::rgb(to_byte(r), to_byte(g), to_byte(b)))
    }

    /// Parses `"#rrggbb"` or `"#rrggbbaa"`. The `#` is optional.
    pub fn from_hex(text: &str) -> Result<Self, Error> {
        let hex = text.strip_prefix('#').unwrap_or(text);
        let valid_length = hex.len() == 6 || hex.len() == 8;

        if !valid_length || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::InvalidColor(
                format!("{text:?} is not a hex color like #3a7bd5").into(),
            ));
        }

        // Safe to unwrap_or(0): we just checked every character is a hex digit.
        let byte = |index: usize| u8::from_str_radix(&hex[index..index + 2], 16).unwrap_or(0);
        let alpha = if hex.len() == 8 { byte(6) } else { 255 };

        Ok(Self::new(byte(0), byte(2), byte(4), alpha))
    }

    /// Formats as lowercase `"#rrggbb"`. Alpha is left out; see [`Rgba::opacity`].
    pub fn to_hex(&self) -> String {
        self.to_string()
    }

    /// Alpha as a number from `0.0` (invisible) to `1.0` (solid).
    pub fn opacity(&self) -> f32 {
        f32::from(self.a) / 255.0
    }
}

/// Prints the color as lowercase `#rrggbb`, the same as [`Rgba::to_hex`].
/// Alpha is left out.
impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Settings that control which colors an avatar may use.
///
/// Start from the defaults and change only what you need:
///
/// ```
/// let colors = ferrav::ColorOptions { saturation: 0.8, ..Default::default() };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ColorOptions {
    /// Allowed hues in degrees (0 = red, 120 = green, 240 = blue). Empty = any hue.
    pub hues: Vec<f32>,

    /// Saturation of the colored shapes: 0.0 = gray, 1.0 = vivid.
    pub saturation: f32,

    /// Lightness of the colored shapes as `(darkest, lightest)`, each 0.0–1.0.
    pub lightness: (f32, f32),

    /// Saturation of the gray shapes. 0.0 = pure gray.
    pub grayscale_saturation: f32,

    /// Lightness of the gray shapes as `(darkest, lightest)`, each 0.0–1.0.
    pub grayscale_lightness: (f32, f32),
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            hues: Vec::new(),
            saturation: 0.5,
            lightness: (0.4, 0.8),
            grayscale_saturation: 0.0,
            grayscale_lightness: (0.3, 0.9),
        }
    }
}

impl ColorOptions {
    /// Checks every value is in range.
    pub fn validate(&self) -> Result<(), Error> {
        let in_unit = |v: f32| (0.0..=1.0).contains(&v);

        if !in_unit(self.saturation) {
            return Err(Error::InvalidColor(
                format!(
                    "saturation {} must be between 0.0 and 1.0",
                    self.saturation
                )
                .into(),
            ));
        }

        if !in_unit(self.grayscale_saturation) {
            return Err(Error::InvalidColor(
                format!(
                    "grayscale_saturation {} must be between 0.0 and 1.0",
                    self.grayscale_saturation
                )
                .into(),
            ));
        }

        for (name, (low, high)) in [
            ("lightness", self.lightness),
            ("grayscale_lightness", self.grayscale_lightness),
        ] {
            if !in_unit(low) || !in_unit(high) || low > high {
                return Err(Error::InvalidColor(
                    format!("{name} ({low}, {high}) must satisfy 0.0 <= low <= high <= 1.0").into(),
                ));
            }
        }

        if let Some(hue) = self.hues.iter().find(|h| !h.is_finite()) {
            return Err(Error::InvalidColor(
                format!("hue {hue} is not a finite number").into(),
            ));
        }

        Ok(())
    }
}

/// The five colors one avatar is drawn with, all made from one hue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Dark shade of the hue.
    pub dark: Rgba,
    /// Middle shade of the hue.
    pub mid: Rgba,
    /// Light shade of the hue.
    pub light: Rgba,
    /// Dark gray (tinted by the hue if `grayscale_saturation` > 0).
    pub dark_gray: Rgba,
    /// Light gray (tinted by the hue if `grayscale_saturation` > 0).
    pub light_gray: Rgba,
}

impl Palette {
    /// Builds the palette for one hue.
    pub fn new(hue: f32, options: &ColorOptions) -> Self {
        let (low, high) = options.lightness;
        let (gray_low, gray_high) = options.grayscale_lightness;
        let sat = options.saturation;
        let gray_sat = options.grayscale_saturation;

        Self {
            dark: Rgba::from_hsl(hue, sat, low),
            mid: Rgba::from_hsl(hue, sat, (low + high) / 2.0),
            light: Rgba::from_hsl(hue, sat, high),
            dark_gray: Rgba::from_hsl(hue, gray_sat, gray_low),
            light_gray: Rgba::from_hsl(hue, gray_sat, gray_high),
        }
    }

    /// Picks a hue with `rng` (from `options.hues` if any are set) and builds the palette.
    pub fn random(rng: &mut Rng, options: &ColorOptions) -> Self {
        let hue = if options.hues.is_empty() {
            rng.next_f32() * 360.0
        } else {
            *rng.pick(&options.hues)
        };

        Self::new(hue, options)
    }

    /// All five colors as an array.
    pub fn all(&self) -> [Rgba; 5] {
        [
            self.dark,
            self.mid,
            self.light,
            self.dark_gray,
            self.light_gray,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsl_primaries() {
        assert_eq!(Rgba::from_hsl(0.0, 1.0, 0.5), Rgba::rgb(255, 0, 0));
        assert_eq!(Rgba::from_hsl(120.0, 1.0, 0.5), Rgba::rgb(0, 255, 0));
        assert_eq!(Rgba::from_hsl(240.0, 1.0, 0.5), Rgba::rgb(0, 0, 255));
        assert_eq!(Rgba::from_hsl(0.0, 0.0, 0.5), Rgba::rgb(128, 128, 128));
    }

    #[test]
    fn hsl_edges() {
        assert_eq!(Rgba::from_hsl(-360.0, 1.0, 0.5), Rgba::rgb(255, 0, 0));
        assert_eq!(Rgba::from_hsl(0.0, 1.0, 0.0), Rgba::BLACK);
        assert_eq!(Rgba::from_hsl(0.0, 1.0, 1.0), Rgba::WHITE);
        // Saturation above 1.0 is clamped to 1.0.
        assert_eq!(Rgba::from_hsl(0.0, 2.0, 0.5), Rgba::rgb(255, 0, 0));
    }

    #[test]
    fn try_from_hsl_rejects_non_finite_values() {
        assert!(Rgba::try_from_hsl(f32::NAN, 1.0, 0.5).is_err());
        assert!(Rgba::try_from_hsl(0.0, f32::INFINITY, 0.5).is_err());
        assert_eq!(Rgba::from_hsl(f32::NAN, 1.0, 0.5), Rgba::BLACK);
    }

    #[test]
    fn hex_round_trip() {
        let color = Rgba::from_hex("#3a7bd5").unwrap();
        assert_eq!(color, Rgba::rgb(0x3a, 0x7b, 0xd5));
        assert_eq!(color.to_hex(), "#3a7bd5");
        assert_eq!(Rgba::from_hex("3a7bd580").unwrap().a, 0x80);
        assert!(Rgba::from_hex("#xyz").is_err());
    }

    #[test]
    fn display_matches_to_hex() {
        let color = Rgba::rgb(0x3a, 0x7b, 0xd5);
        assert_eq!(color.to_string(), "#3a7bd5");
        assert_eq!(color.to_string(), color.to_hex());
    }

    #[test]
    fn options_are_validated() {
        assert!(ColorOptions::default().validate().is_ok());

        let bad = ColorOptions {
            saturation: 1.5,
            ..Default::default()
        };
        assert!(bad.validate().is_err());

        let backwards = ColorOptions {
            lightness: (0.9, 0.1),
            ..Default::default()
        };
        assert!(backwards.validate().is_err());
    }

    #[test]
    fn restricted_hue_is_used() {
        let options = ColorOptions {
            hues: vec![0.0],
            ..Default::default()
        };
        let palette = Palette::random(&mut Rng::new(1), &options);
        assert!(palette.mid.r > palette.mid.g && palette.mid.r > palette.mid.b);
    }
}