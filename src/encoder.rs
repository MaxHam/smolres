use jpeg_decoder::ImageInfo;
use jpeg_encoder::{ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn encode(vec: Vec<u8>, metadata: ImageInfo, output_file_path: &PathBuf) -> () {
    // Encodes the pixel vector back to an jpeg file and also saves it to a path
    let output = File::create(output_file_path).unwrap();
    let encoder = Encoder::new(BufWriter::new(output), 100);
    encoder
        .encode(
            &vec,
            metadata.width as u16,
            metadata.height as u16,
            ColorType::Rgb,
        )
        .expect("JPEG encoding failed");
}
