use std::{
    cmp::{max, min},
    path::Path,
};

use image::{DynamicImage, GenericImageView, Pixel, imageops::FilterType};

use crate::{Error, Result};

/// Simple character set (~10 characters).
pub const CHAR_SET_SIMPLE: &str = " .'`-_:;+=*%#@";

/// Medium character set (~25 characters).
pub const CHAR_SET_MEDIUM: &str = " .'`-_:,;^!i?l~+/*()[]{}<>%#&$@";

/// Detailed character set (~50 characters).
pub const CHAR_SET_DETAILED: &str = " .'-,_`\":;!~*^=+|?/()[]{}<>ilrcvunxzjftmwqpdbaohk#$&%@";

/// Very detailed character set (~100 characters).
pub const CHAR_SET_VERY_DETAILED: &str = " `.-,:_`';\"^!|*\\/()[]{}?i~+><1lIctrJFnvuxLzsfjymeowqpkSadZbghVCE753YUROXG0PDK26948HQMNWB$%&#@";

/// Configuration for ASCII art conversion.
pub struct AsciiArtConfig {
    /// Target width in characters.
    pub width: u32,
    /// Maximum target height in characters.
    pub height: u32,
    /// Compensation factor for character aspect ratio (height/width). Must be > 0.
    pub aspect_ratio_compensation: f32,
    /// Filter type for resizing.
    pub resize_filter: FilterType,
    /// Character set for brightness mapping.
    pub char_set: Vec<char>,
    /// Exponent for brightness adjustment (0.0-1.0 before mapping).
    pub brightness_exponent: f32,
}

impl Default for AsciiArtConfig {
    #[inline]
    fn default() -> Self {
        AsciiArtConfig {
            width: 100,
            height: 50,
            aspect_ratio_compensation: 2.0,
            resize_filter: FilterType::Lanczos3,
            char_set: CHAR_SET_DETAILED.chars().collect::<Vec<char>>(),
            brightness_exponent: 0.25,
        }
    }
}

fn _image_to_ascii_core(img: DynamicImage, config: &AsciiArtConfig) -> Result<String> {
    if config.char_set.is_empty() {
        return Err(Error::ValidationError {
            expected: "Non-empty character set".to_owned(),
            actual:   "Empty character set".to_owned(),
            context:  Some("ASCII conversion requires at least one character".to_owned()),
        });
    }
    if config.aspect_ratio_compensation <= 0.0 {
        return Err(Error::ValidationError {
            expected: "Positive aspect ratio compensation".to_owned(),
            actual:   format!("Compensation factor: {}", config.aspect_ratio_compensation),
            context:  Some("Aspect ratio compensation must be greater than 0".to_owned()),
        });
    }
    if config.width == 0 {
        return Err(Error::ValidationError {
            expected: "Positive width".to_owned(),
            actual:   "Width: 0".to_owned(),
            context:  Some("Target width must be greater than 0".to_owned()),
        });
    }
    if img.height() == 0 {
        return Err(Error::ValidationError {
            expected: "Non-zero image height".to_owned(),
            actual:   "Image height: 0".to_owned(),
            context:  Some("Input image height cannot be zero".to_owned()),
        });
    }

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let scaled_width = config.width;
    let calculated_scaled_height =
        (config.width as f32 / aspect_ratio / config.aspect_ratio_compensation).round() as u32;
    let scaled_height = max(1, min(calculated_scaled_height, config.height));

    let resized_img = img.resize_exact(scaled_width, scaled_height, config.resize_filter);

    let mut result = String::with_capacity(((scaled_width + 1) * scaled_height) as usize);
    let num_chars = config.char_set.len();
    let num_chars_f = num_chars as f32;

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = resized_img.get_pixel(x, y);
            let brightness = pixel_brightness(pixel);
            let adjusted_brightness = brightness.powf(config.brightness_exponent);
            let char_f_index = (1.0 - adjusted_brightness) * num_chars_f;
            let mut char_index = char_f_index.floor() as usize;

            if char_index >= num_chars {
                char_index = num_chars - 1;
            }

            result.push(config.char_set[char_index]);
        }
        result.push('\n');
    }

    Ok(result)
}

/// Converts an image to ASCII art using a given configuration.
pub fn image_to_ascii_configurable<P>(path: P, config: &AsciiArtConfig) -> Result<String>
where
    P: AsRef<Path>,
{
    let img_path = path.as_ref();
    let img = image::open(img_path).map_err(|e| {
        Error::IoError(format!(
            "Could not open or decode image file at path '{}': {}",
            img_path.display(),
            e,
        ))
    })?;

    _image_to_ascii_core(img, config)
}

/// Converts an image to ASCII art with specified width, height, and character set string.
/// Uses default values for other configuration options.
#[inline]
pub fn image_to_ascii<P, C>(path: P, width: u32, height: u32, char_set: C) -> Result<String>
where
    P: AsRef<Path>,
    C: AsRef<str>,
{
    let chars: Vec<char> = char_set.as_ref().chars().collect();
    let config = AsciiArtConfig {
        width,
        height,
        char_set: chars,
        ..Default::default()
    };

    image_to_ascii_configurable(path, &config)
}

/// Calculates the brightness of a pixel.
#[inline]
pub fn pixel_brightness<P: Pixel<Subpixel = u8>>(pixel: P) -> f32 {
    let channels = pixel.to_rgb();
    let r = channels[0] as f32 / 255.0;
    let g = channels[1] as f32 / 255.0;
    let b = channels[2] as f32 / 255.0;

    (0.2126 * r + 0.7152 * g + 0.0722 * b).min(1.0)
}
