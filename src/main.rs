mod cli;
mod decoder;
mod encoder;
mod interpolation;

use clap::Parser;
use cli::{Args, default_output_path};
use decoder::decode;
use encoder::encode;
use interpolation::{down_sample_average_area, up_sample_nearest_neighbor};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserFacingError {
    #[error("Failed to transform image: {0}")]
    DownsamplingError(#[from] interpolation::DownsamplingError),
    #[error("Failed to transform image: {0}")]
    UpsamplingError(#[from] interpolation::UpsamplingError),
    #[error("Unknown error: {0}")]
    Other(String),
}

fn main() -> Result<(), UserFacingError> {
    let args = Args::parse();

    let output = args
        .output
        .clone()
        .unwrap_or_else(|| default_output_path(&args.input, args.resolution));

    let (pixel_vec, metadata) = decode(&args.input);

    let downsampled_pixel_vec: Vec<u8> = down_sample_average_area(
        pixel_vec,
        metadata.width.into(),
        metadata.height.into(),
        args.resolution.into(),
        args.resolution.into(),
        metadata.pixel_format,
    )?;
    let upsampled_pixel_vec: Vec<u8> = up_sample_nearest_neighbor(
        downsampled_pixel_vec,
        args.resolution.into(),
        args.resolution.into(),
        metadata.height.into(),
        metadata.width.into(),
        metadata.pixel_format,
    )?;

    encode(upsampled_pixel_vec, metadata.height, metadata.width, output);
    Ok(())
}
