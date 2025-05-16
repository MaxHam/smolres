extern crate jpeg_decoder as jpeg;

use jpeg_decoder::{Decoder, ImageInfo};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn decode(file: &PathBuf) -> (Vec<u8>, ImageInfo) {
    let file = File::open(file).expect("failed to open file");
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata: ImageInfo = decoder.info().unwrap();
    return (pixels, metadata);
}
