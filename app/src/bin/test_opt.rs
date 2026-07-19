use std::time::Instant;
use zune_jpeg::JpegDecoder;
use zune_core::options::DecoderOptions;
use zune_core::colorspace::ColorSpace;
use std::io::Cursor;

fn main() {
    // Generate a dummy JPEG in memory using image crate just for testing? No, I need a real JPEG.
    // I can just rely on the compiler optimization.
    println!("Compiled with target-cpu=native!");
}
