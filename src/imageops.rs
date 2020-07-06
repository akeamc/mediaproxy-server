use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use mediaproxy_common::{OutputFormat, ResizeStrategy};
use num::clamp;
use std::str::FromStr;
use std::time::Instant;

use crate::performance::Performance;

pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096

pub struct ResizeResponse {
    pub img: DynamicImage,
    pub performance: Performance,
}

pub fn resize(
    img: &DynamicImage,
    strategy: ResizeStrategy,
    width: Option<u32>,
    height: Option<u32>,
) -> ResizeResponse {
    let start = Instant::now();

    let nwidth = clamp(width.unwrap_or_else(|| img.width()), 1, MAX_IMAGE_SIZE);
    let nheight = clamp(height.unwrap_or_else(|| img.height()), 1, MAX_IMAGE_SIZE);

    let resized: DynamicImage = match strategy {
        ResizeStrategy::Contain => img.thumbnail(nwidth, nheight),
        ResizeStrategy::Crop => {
            img.resize_to_fill(nwidth, nheight, image::imageops::FilterType::Triangle)
        }
        ResizeStrategy::Stretch => img.thumbnail_exact(nwidth, nheight),
    };

    ResizeResponse {
        img: resized,
        performance: Performance {
            elapsed_ns: start.elapsed().as_nanos(),
        },
    }
}

pub fn get_media_type(output: &OutputFormat) -> mime::Mime {
    match output {
        OutputFormat::Jpeg => mime::IMAGE_JPEG,
        OutputFormat::Png => mime::IMAGE_PNG,
        OutputFormat::WebP => mime::Mime::from_str("image/webp").unwrap(),
        OutputFormat::Gif => mime::IMAGE_GIF,
    }
}

pub mod to_bytes {
    use super::*;

    fn default_image_format(
        img: &DynamicImage,
        format: ImageOutputFormat,
    ) -> Result<Vec<u8>, image::ImageError> {
        let mut result: Vec<u8> = Vec::new();
        img.write_to(&mut result, format)?;
        Ok(result)
    }

    fn webp(img: &DynamicImage, quality: f32) -> Result<Vec<u8>, image::ImageError> {
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba();
        let encoded = webp::Encoder::from_rgba(&rgba, width, height).encode(quality);
        Ok(encoded.to_vec())
    }

    pub fn image(
        img: &DynamicImage,
        format: OutputFormat,
    ) -> Result<Vec<u8>, image::error::ImageError> {
        match format {
            OutputFormat::Jpeg => default_image_format(img, ImageOutputFormat::Jpeg(80)),
            OutputFormat::Png => default_image_format(img, ImageOutputFormat::Png),
            OutputFormat::WebP => webp(img, 90.0),
            OutputFormat::Gif => default_image_format(img, ImageOutputFormat::Gif),
        }
    }
}
