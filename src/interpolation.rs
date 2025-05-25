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
        /*
         * Uses average area downsampling interpolation
         *
         * Given kxk pixel block
         * Block = [
         * [R₁,G₁,B₁], [R₂,G₂,B₂], ..., [Rₖ,Gₖ,Bₖ],
         * ...
         * ]
         *
         * Compute average
         * R_avg = (R₁ + R₂ + ... + Rₖ) / (k * k)
         * G_avg = ...
         * B_avg = ...
         *
         * P_avg = [R_avg, G_avg, B_avg]
         */

        if target_height > src_height || target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target Height {} > Source Height {}. For the downsampling to be reliable the target resolution has to be lower than the source resolution",
                    target_height, src_height,
                ),
            ));
        }
        if target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target Width {} > Source Width {}. For the downsampling to be reliable the target resolution has to be lower than the source resolution",
                    target_width, src_width,
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
                let mut r_sum: usize = 0;
                let mut g_sum: usize = 0;
                let mut b_sum: usize = 0;
                for y in 0..block_size_y {
                    for x in 0..block_size_x {
                        let pixel_x = block_x * block_size_x + x;
                        let pixel_y = block_y * block_size_y + y;
                        let idx = (pixel_y * src_width + pixel_x) * pixel_bytes;

                        r_sum += src_pixels[idx] as usize;
                        g_sum += src_pixels[idx + 1] as usize;
                        b_sum += src_pixels[idx + 2] as usize;
                    }
                }
                // compute the average
                let count = block_size_x * block_size_y;
                target_pixels.push((r_sum / &count) as u8);
                target_pixels.push((g_sum / &count) as u8);
                target_pixels.push((b_sum / &count) as u8);
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
        let result = NearestNeighborInterpolation.upsample(
            src_pixels,
            src_width,
            src_height,
            target_width,
            target_height,
            pixel_format,
        )?;
        Ok(result)
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
        // original_height = input_image.height
        // original_width = input_image.width
        //
        // # Create an empty image with new dimensions
        // output_image = create_empty_image(new_width, new_height)
        //
        // # Calculate the scale factors
        // x_scale = original_width / new_width
        // y_scale = original_height / new_height
        //
        // for y_new from 0 to new_height - 1:
        //     for x_new from 0 to new_width - 1:
        //
        // # Map coordinates in the output image back to the nearest input pixel
        // x_old = floor(x_new * x_scale)
        // y_old = floor(y_new * y_scale)
        //
        // # Assign the nearest pixel value
        // output_image[y_new][x_new] = input_image[y_old][x_old]
        // TODO: implement this
        if target_height > src_height || target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target Height {} > Source Height {}. For the downsampling to be reliable the target resolution has to be lower than the source resolution",
                    target_height, src_height,
                ),
            ));
        }
        if target_width > src_width {
            return Err(InterpolationError::DownsampleTargetLargerThanSource(
                format!(
                    "Target Width {} > Source Width {}. For the downsampling to be reliable the target resolution has to be lower than the source resolution",
                    target_width, src_width,
                ),
            ));
        }
        let pixel_bytes: usize = pixel_format
            .pixel_bytes()
            .try_into()
            .map_err(|_e: Infallible| InterpolationError::ImageMetadataResolve)?;

        let mut target_pixels: Vec<u8> = vec![0u8; target_width * target_height * pixel_bytes];
        let scale_x = src_width / target_width;
        let scale_y = src_height / target_height;
        for y in 0..target_height {
            for x in 0..target_width {
                // clamp the coords so no overflow at the edges occurs.
                let original_x = (((x * scale_x) as f64).floor() as usize).min(src_width - 1);
                let original_y = (((y * scale_y) as f64).floor() as usize).min(src_height - 1);

                let src_idx = (original_y * src_width + original_x) * pixel_bytes;
                let out_idx = (y * target_width + x) * pixel_bytes;

                target_pixels[out_idx] = src_pixels[src_idx];
                target_pixels[out_idx + 1] = src_pixels[src_idx + 1];
                target_pixels[out_idx + 2] = src_pixels[src_idx + 2];
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
        let mut target_pixels: Vec<u8> = vec![0u8; target_width * target_height * pixel_bytes];

        if target_pixels.len() <= src_pixels.len() {
            return Err(InterpolationError::UpsampleSourceLargerThanTarget(format!(
                "The length of the source pixel vector is {} and the length of the target pixel vector is {}. For a reliable upsampling the target pixel vector needs to be greater.",
                src_pixels.len(),
                target_pixels.len(),
            )));
        }
        let scale_x = target_width / src_width;
        let scale_y = target_height / src_height;
        for y in 0..target_height {
            for x in 0..target_width {
                // clamp the coords so no overflow at the edges occurs.
                let original_x = (((x / scale_x) as f64).floor() as usize).min(src_width - 1);
                let original_y = (((y / scale_y) as f64).floor() as usize).min(src_height - 1);

                let src_idx = (original_y * src_width + original_x) * pixel_bytes;
                let out_idx = (y * target_width + x) * pixel_bytes;

                target_pixels[out_idx] = src_pixels[src_idx];
                target_pixels[out_idx + 1] = src_pixels[src_idx + 1];
                target_pixels[out_idx + 2] = src_pixels[src_idx + 2];
            }
        }

        Ok(target_pixels)
    }
}

pub fn run_interpolation(
    algo: &dyn InterpolationAlgorithm,
    src: Vec<u8>,
    target_resolution: u16,
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
    algo.upsample(
        downsampled_pixels,
        target_resolution.into(),
        target_resolution.into(),
        src_width.into(),
        src_height.into(),
        metadata.pixel_format,
    )
}
