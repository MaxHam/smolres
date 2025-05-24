mod cli;
mod decoder;
mod encoder;
mod interpolation;

use clap::Parser;
use cli::{Args, default_output_path};
use decoder::decode;
use encoder::encode;
use interpolation::{NearestNeighborInterpolation, run_interpolation};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserFacingError {
    #[error("Failed to interpolate image: {0}")]
    InterpolationError(#[from] interpolation::InterpolationError),
}

fn run(args: Args) -> Result<(), UserFacingError> {
    let output = args
        .output
        .clone()
        .unwrap_or_else(|| default_output_path(&args.input, args.resolution));

    let (pixel_vec, metadata) = decode(&args.input);

    let interpolated_pixels: Vec<u8> = run_interpolation(
        &NearestNeighborInterpolation,
        pixel_vec,
        args.resolution,
        metadata,
    )?;
    println!("{}", interpolated_pixels[0]);
    encode(interpolated_pixels, metadata.height, metadata.width, output);
    Ok(())
}

fn main() -> Result<(), UserFacingError> {
    let args = Args::parse();
    let _ = run(args);
    Ok(())
}
