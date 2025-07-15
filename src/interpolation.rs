use std::convert::Infallible;

use jpeg_decoder::{ImageInfo, PixelFormat};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpolationError {
    #[error("Target dimensions are larger than source dimensions: {0}")]
    DownsampleTargetLargerThanSource(String),

    #[error("Source dimensions are larger than target dimensions: {0}")]
    UpsampleSourceLargerThanTarget(String),

    #[error("Failed to resolve image metadata")]
    ImageMetadataResolve,
}
pub trait InterpolationAlgorithm {
    fn downsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError>;

    fn upsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError>;
}

pub struct AverageAreaInterpolation;
pub struct NearestNeighborInterpolation;

impl InterpolationAlgorithm for AverageAreaInterpolation {
    fn downsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError> {
        if target_height > src_height || target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target resolution ({}, {}) > Source resolution ({}, {})",
                    target_width, target_height, src_width, src_height
                ),
            ));
        }

        let pixel_bytes: usize = pixel_format
            .pixel_bytes()
            .try_into()
            .map_err(|_e: Infallible| InterpolationError::ImageMetadataResolve)?;

        let block_size_x = src_width / target_width;
        let block_size_y = src_height / target_height;

        let mut target_pixels: Vec<u8> =
            Vec::with_capacity(target_height * target_width * pixel_bytes);

        for block_y in 0..target_height {
            for block_x in 0..target_width {
                let mut sums = vec![0usize; pixel_bytes];

                for y in 0..block_size_y {
                    for x in 0..block_size_x {
                        let pixel_x = block_x * block_size_x + x;
                        let pixel_y = block_y * block_size_y + y;
                        let idx = (pixel_y * src_width + pixel_x) * pixel_bytes;
                        for channel in 0..pixel_bytes {
                            sums[channel] += src_pixels[idx + channel] as usize;
                        }
                    }
                }

                let count = block_size_x * block_size_y;
                for channel_sum in sums {
                    target_pixels.push((channel_sum / count) as u8);
                }
            }
        }

        Ok(target_pixels)
    }

    fn upsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError> {
        let pixel_bytes: usize = pixel_format
            .pixel_bytes()
            .try_into()
            .map_err(|_e: Infallible| InterpolationError::ImageMetadataResolve)?;

        let mut target_pixels = Vec::with_capacity(target_height * target_width * pixel_bytes);
        let scale_x = src_width as f64 / target_width as f64;
        let scale_y = src_height as f64 / target_height as f64;

        for y_target in 0..target_height {
            for x_target in 0..target_width {
                let x_start = (x_target as f64 * scale_x).floor() as usize;
                let x_end = ((x_target + 1) as f64 * scale_x).ceil() as usize;

                let y_start = (y_target as f64 * scale_y).floor() as usize;
                let y_end = ((y_target + 1) as f64 * scale_y).ceil() as usize;

                let mut sums = vec![0usize; pixel_bytes];
                let mut count = 0;

                for y in y_start..y_end.min(src_height) {
                    for x in x_start..x_end.min(src_width) {
                        let idx = (y * src_width + x) * pixel_bytes;
                        for c in 0..pixel_bytes {
                            sums[c] += src_pixels[idx + c] as usize;
                        }
                        count += 1;
                    }
                }

                for sum in sums {
                    target_pixels.push((sum / count) as u8);
                }
            }
        }

        Ok(target_pixels)
    }
}

impl InterpolationAlgorithm for NearestNeighborInterpolation {
    fn downsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError> {
        if target_height > src_height || target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target resolution ({}, {}) > Source resolution ({}, {})",
                    target_width, target_height, src_width, src_height
                ),
            ));
        }

        let pixel_bytes = pixel_format.pixel_bytes() as usize;
        let mut target_pixels = vec![0u8; target_width * target_height * pixel_bytes];

        let scale_x = src_width as f64 / target_width as f64;
        let scale_y = src_height as f64 / target_height as f64;

        for y in 0..target_height {
            for x in 0..target_width {
                let src_x = (x as f64 * scale_x).floor() as usize;
                let src_y = (y as f64 * scale_y).floor() as usize;

                let src_idx = (src_y * src_width + src_x) * pixel_bytes;
                let out_idx = (y * target_width + x) * pixel_bytes;

                target_pixels[out_idx..out_idx + pixel_bytes]
                    .copy_from_slice(&src_pixels[src_idx..src_idx + pixel_bytes]);
            }
        }

        Ok(target_pixels)
    }

    fn upsample(
        &self,
        src_pixels: Vec<u8>,
        src_width: usize,
        src_height: usize,
        target_width: usize,
        target_height: usize,
        pixel_format: PixelFormat,
    ) -> Result<Vec<u8>, InterpolationError> {
        let pixel_bytes = pixel_format.pixel_bytes() as usize;
        let mut target_pixels = vec![0u8; target_width * target_height * pixel_bytes];

        if target_pixels.len() <= src_pixels.len() {
            return Err(InterpolationError::UpsampleSourceLargerThanTarget(format!(
                "Source pixel vec is {}, target vec is {}",
                src_pixels.len(),
                target_pixels.len()
            )));
        }

        let scale_x = src_width as f64 / target_width as f64;
        let scale_y = src_height as f64 / target_height as f64;

        for y in 0..target_height {
            for x in 0..target_width {
                let src_x = (x as f64 * scale_x).floor() as usize;
                let src_y = (y as f64 * scale_y).floor() as usize;

                let src_idx = (src_y * src_width + src_x) * pixel_bytes;
                let out_idx = (y * target_width + x) * pixel_bytes;

                target_pixels[out_idx..out_idx + pixel_bytes]
                    .copy_from_slice(&src_pixels[src_idx..src_idx + pixel_bytes]);
            }
        }

        Ok(target_pixels)
    }
}

pub fn reduce_bit_depth(pixels: &mut [u8], bit_depth: u8) {
    let levels = 1 << bit_depth;
    let step = (256u16 / levels as u16) as u8;
    for byte in pixels.iter_mut() {
        *byte = (*byte / step) * step;
    }
}

pub fn run_interpolation(
    algo: &dyn InterpolationAlgorithm,
    src: Vec<u8>,
    target_resolution: u16,
    target_bit_depth: u8,
    metadata: ImageInfo,
) -> Result<Vec<u8>, InterpolationError> {
    let src_width = metadata.width;
    let src_height = metadata.height;
    let downsampled_pixels = algo.downsample(
        src,
        src_width.into(),
        src_height.into(),
        target_resolution.into(),
        target_resolution.into(),
        metadata.pixel_format,
    )?;
    let mut target_pixels = algo.upsample(
        downsampled_pixels,
        target_resolution.into(),
        target_resolution.into(),
        src_width.into(),
        src_height.into(),
        metadata.pixel_format,
    )?;
    reduce_bit_depth(&mut target_pixels, target_bit_depth);
    Ok(target_pixels)
}

#[cfg(test)]
mod tests {
    use super::{NearestNeighborInterpolation, reduce_bit_depth, run_interpolation};
    use crate::interpolation::AverageAreaInterpolation;
    use jpeg_decoder::{CodingProcess, ImageInfo, PixelFormat};

    #[test]
    fn test_nearest_neighbor_interpolation() {
        let width = 4;
        let height = 4;
        let pixel_format = 3;
        let mock_pixels: Vec<u8> = vec![128u8; width * height * pixel_format];
        let original_pixels = mock_pixels.clone();
        let target_bit_depth = 8;
        let metadata = ImageInfo {
            width: width as u16,
            height: height as u16,
            pixel_format: PixelFormat::RGB24,
            coding_process: CodingProcess::DctSequential,
        };
        let target_resolution = 2;
        let result_pixels = run_interpolation(
            &NearestNeighborInterpolation,
            mock_pixels,
            target_resolution,
            target_bit_depth,
            metadata,
        )
        .unwrap();
        assert_eq!(result_pixels.len(), original_pixels.len());
    }

    #[test]
    fn test_average_area_interpolation() {
        let width = 4;
        let height = 4;
        let pixel_format = 3;
        let mock_pixels: Vec<u8> = vec![128u8; width * height * pixel_format];
        let original_pixels = mock_pixels.clone();
        let metadata = ImageInfo {
            width: width as u16,
            height: height as u16,
            pixel_format: PixelFormat::RGB24,
            coding_process: CodingProcess::DctSequential,
        };
        let target_resolution = 2;
        let target_bit_depth = 8;
        let result_pixels = run_interpolation(
            &AverageAreaInterpolation,
            mock_pixels,
            target_resolution,
            target_bit_depth,
            metadata,
        )
        .unwrap();
        assert_eq!(result_pixels.len(), original_pixels.len());
    }

    #[test]
    fn test_reduce_bit_depth() {
        let mut pixels = vec![255, 128, 64, 32, 16, 0];

        // 2-bit depth -> 4 levels -> step = 64
        // Expected values: 255 -> 192, 128 -> 128, 64 -> 64, etc.
        // (x / 64) * 64 = quantized value
        reduce_bit_depth(&mut pixels, 2);

        let expected = vec![192, 128, 64, 0, 0, 0];
        assert_eq!(pixels, expected);
    }
}
