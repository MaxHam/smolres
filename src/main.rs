mod cli;
mod decoder;
mod encoder;
mod interpolation;

use clap::Parser;
use cli::{Algorithm, Args, default_output_path};
use decoder::decode;
use encoder::encode;
use interpolation::{
    AverageAreaInterpolation, InterpolationAlgorithm, NearestNeighborInterpolation,
    run_interpolation,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserFacingError {
    #[error("Failed to interpolate image: {0}")]
    InterpolationError(#[from] interpolation::InterpolationError),
}

pub fn run(args: Args) -> Result<(), UserFacingError> {
    let algo = args.algorithm.unwrap_or(Algorithm::AverageArea);

    let chosen_interpolation_algo: &dyn InterpolationAlgorithm = match algo {
        Algorithm::AverageArea => &AverageAreaInterpolation,
        Algorithm::Nearestneighbor => &NearestNeighborInterpolation,
    };

    let output = args
        .output
        .clone()
        .unwrap_or_else(|| default_output_path(&args.input, args.resolution, algo));

    let (pixel_vec, metadata) = decode(&args.input);

    let interpolated_pixels: Vec<u8> = run_interpolation(
        chosen_interpolation_algo,
        pixel_vec,
        args.resolution,
        args.bit_depth,
        metadata,
    )?;
    encode(interpolated_pixels, metadata.height, metadata.width, output);
    Ok(())
}

fn main() -> Result<(), UserFacingError> {
    let args = Args::parse();
    let _ = run(args);
    Ok(())
}
#[cfg(test)]
mod tests {

    use jpeg_decoder::Decoder;

    use crate::cli::{Algorithm, Args};
    use crate::run;
    use std::fs::File;
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn test_run_method_average_area() {
        let input_path = PathBuf::from("examples/horse_3.jpeg"); // Ensure this file exists
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("output.jpeg");
        let args = Args {
            input: input_path.clone(),
            output: Some(output_path.clone()),
            resolution: 16,
            bit_depth: 4,
            algorithm: Some(Algorithm::AverageArea),
        };

        run(args).expect("run() should succeed");

        assert!(output_path.exists(), "Output image was not created");

        let mut input_file = File::open(&input_path).expect("Failed to open input image");
        let mut decoder = Decoder::new(&mut input_file);
        let input_pixels = decoder.decode().expect("Failed to decode input image");

        let mut output_file = File::open(&output_path).expect("Failed to open output image");
        let mut decoder_out = Decoder::new(&mut output_file);
        let output_pixels = decoder_out.decode().expect("Failed to decode output image");
        assert_eq!(
            input_pixels.len(),
            output_pixels.len(),
            "Input and output images have different pixel counts"
        );

        // Clean up
        fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_run_method_nearest_neighbor() {
        let input_path = PathBuf::from("examples/horse.jpeg"); // Ensure this file exists
        let temp_dir = env::temp_dir();
        let output_path = temp_dir.join("output.jpeg");
        let args = Args {
            input: input_path.clone(),
            output: Some(output_path.clone()),
            resolution: 16,
            bit_depth: 4,
            algorithm: Some(Algorithm::Nearestneighbor),
        };

        run(args).expect("run() should succeed");

        assert!(output_path.exists(), "Output image was not created");

        let mut input_file = File::open(&input_path).expect("Failed to open input image");
        let mut decoder = Decoder::new(&mut input_file);
        let input_pixels = decoder.decode().expect("Failed to decode input image");

        let mut output_file = File::open(&output_path).expect("Failed to open output image");
        let mut decoder_out = Decoder::new(&mut output_file);
        let output_pixels = decoder_out.decode().expect("Failed to decode output image");

        assert_eq!(
            input_pixels.len(),
            output_pixels.len(),
            "Input and output images have different pixel counts"
        );

        // Clean up
        fs::remove_file(output_path).unwrap();
    }
}
