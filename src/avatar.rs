//! The public face of the crate: [`Avatar`], [`AvatarBuilder`] and [`Mode`].

use std::sync::Arc;

use crate::color::{ColorOptions, Palette, Rgba};
use crate::error::Error;
use crate::geometry::Point;
use crate::hash::fnv1a;
use crate::raster::render_rgba;
use crate::rng::{Rng, random_seed};
use crate::scene::Scene;
use crate::style::{Geometric, Style};
use crate::svg::render_svg;

/// Smallest allowed avatar size, in pixels.
pub const MIN_SIZE: u32 = 16;

/// Largest allowed avatar size, in pixels.
pub const MAX_SIZE: u32 = 4096;

/// How text is turned into a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Mode {
    /// Same text, same avatar: every time, on every machine.
    #[default]
    Deterministic,

    /// Same text, a new avatar on every call. Save [`Avatar::seed`] if you
    /// want to show the same avatar again later.
    Random,
}

impl Mode {
    /// Turns text into a seed according to this mode.
    ///
    /// [`Mode::Deterministic`] always gives the same seed for the same text.
    /// [`Mode::Random`] mixes in a fresh random number on every call.
    pub fn seed_for(self, text: &str) -> u64 {
        match self {
            Self::Deterministic => fnv1a(text),
            Self::Random => fnv1a(text) ^ random_seed(),
        }
    }
}

/// A finished avatar, ready to turn into SVG or PNG.
///
/// The same seed and the same settings always give the same avatar.
#[derive(Debug, Clone, PartialEq)]
pub struct Avatar {
    seed: u64,
    scene: Scene,
}

impl Avatar {
    /// Same text, same avatar, using the default settings.
    pub fn new(text: &str) -> Self {
        AvatarBuilder::new()
            .text(text)
            .build()
            .expect("default avatar settings are valid")
    }

    /// A new random avatar on every call, using the default settings.
    /// The text is mixed in, but the result is different each time.
    pub fn random(text: &str) -> Self {
        AvatarBuilder::new()
            .text(text)
            .mode(Mode::Random)
            .build()
            .expect("default avatar settings are valid")
    }

    /// Rebuilds the avatar for a saved seed, using the default settings.
    ///
    /// If the avatar was made with custom settings, use the same builder
    /// plus [`AvatarBuilder::seed`] instead.
    pub fn from_seed(seed: u64) -> Self {
        AvatarBuilder::new()
            .seed(seed)
            .build()
            .expect("default avatar settings are valid")
    }

    /// Starts a builder for full control over every setting.
    pub fn builder() -> AvatarBuilder {
        AvatarBuilder::new()
    }

    /// The seed this avatar was made from. Store it to get the same avatar back.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Width and height in pixels.
    pub fn size(&self) -> u32 {
        self.scene.size
    }

    /// The shapes that make up this avatar, for writing your own renderer.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// The avatar as an SVG document.
    pub fn to_svg(&self) -> String {
        render_svg(&self.scene)
    }

    /// The avatar as raw RGBA pixels (4 bytes per pixel, row by row).
    pub fn to_rgba(&self) -> Vec<u8> {
        render_rgba(&self.scene)
    }

    /// The avatar as PNG file bytes. Needs the `png` feature (on by default).
    #[cfg(feature = "png")]
    pub fn to_png(&self) -> Vec<u8> {
        crate::png::encode_png(self.size(), self.size(), &self.to_rgba())
    }
}

/// Configures and builds an [`Avatar`]. Create one with [`Avatar::builder`].
///
/// Every setting has a sensible default, so set only what you need.
/// One configured builder can make many avatars:
///
/// ```
/// use ferrav::{Avatar, Mode, Pixel};
///
/// // Configure once...
/// let settings = Avatar::builder().size(64).style(Pixel::default());
///
/// // ...then build as many avatars as you like.
/// let alice = settings.clone().text("alice").build()?;
/// let random = settings.clone().text("bob").mode(Mode::Random).build()?;
///
/// // Same settings + saved seed = the same avatar back.
/// assert_eq!(settings.seed(random.seed()).build()?, random);
/// # Ok::<(), ferrav::Error>(())
/// ```
#[derive(Debug, Clone)]
#[must_use = "a builder does nothing until you call .build()"]
pub struct AvatarBuilder {
    text: String,
    mode: Mode,
    seed: Option<u64>,
    size: u32,
    padding: f32,
    background: Option<Rgba>,
    colors: ColorOptions,
    style: Arc<dyn Style>,
}

impl Default for AvatarBuilder {
    fn default() -> Self {
        Self {
            text: String::new(),
            mode: Mode::default(),
            seed: None,
            size: 128,
            padding: 0.08,
            background: None,
            colors: ColorOptions::default(),
            style: Arc::new(Geometric),
        }
    }
}

impl AvatarBuilder {
    /// A builder with every setting at its default.
    pub fn new() -> Self {
        Self::default()
    }

    /// The text to make an avatar for, such as a username or email.
    /// Default: empty.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Deterministic (default) or random. See [`Mode`].
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Use this exact seed. Overrides `text` and `mode`.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Width and height in pixels, from [`MIN_SIZE`] (16) to [`MAX_SIZE`] (4096).
    /// Default: 128.
    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    /// Empty space around the drawing, as a fraction of the size
    /// (0.0 up to, not including, 0.5). Default: 0.08.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Background color. Default: transparent.
    pub fn background(mut self, color: Rgba) -> Self {
        self.background = Some(color);
        self
    }

    /// Which colors may be used. Default: [`ColorOptions::default()`].
    pub fn colors(mut self, colors: ColorOptions) -> Self {
        self.colors = colors;
        self
    }

    /// The drawing style: [`Geometric`] (default), [`Pixel`](crate::Pixel), or your own.
    pub fn style(mut self, style: impl Style + 'static) -> Self {
        self.style = Arc::new(style);
        self
    }

    /// Checks the settings and makes the avatar.
    ///
    /// This borrows the builder, so the same builder can make many avatars.
    ///
    /// # Errors
    ///
    /// Returns an error if the size, padding or color settings are out of range.
    pub fn build(&self) -> Result<Avatar, Error> {
        self.validate()?;

        let seed = self.seed.unwrap_or_else(|| self.mode.seed_for(&self.text));

        Ok(self.build_from_seed(seed))
    }

    fn validate(&self) -> Result<(), Error> {
        if !(MIN_SIZE..=MAX_SIZE).contains(&self.size) {
            return Err(Error::InvalidSize {
                size: self.size,
                min: MIN_SIZE,
                max: MAX_SIZE,
            });
        }

        if !(0.0..0.5).contains(&self.padding) {
            return Err(Error::InvalidPadding(self.padding));
        }

        self.colors.validate()
    }

    /// The whole pipeline: seed -> rng -> palette -> style -> scene.
    fn build_from_seed(&self, seed: u64) -> Avatar {
        let mut rng = Rng::new(seed);
        let palette = Palette::random(&mut rng, &self.colors);
        let shapes = self.style.draw(&mut rng, &palette);

        // Styles draw in a 0..1 square. Scale that square to fill the area
        // inside the padding, and shift it right and down past the padding.
        let size = self.size as f32;
        let padding = size * self.padding;
        let offset = Point::new(padding, padding);
        let scale = size - padding * 2.0;

        let shapes = shapes
            .into_iter()
            .map(|shape| shape.transform(offset, scale))
            .collect();

        Avatar {
            seed,
            scene: Scene {
                size: self.size,
                background: self.background,
                shapes,
            },
        }
    }
}
