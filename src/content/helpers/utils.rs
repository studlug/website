use std::io::Write;
use std::fs;

pub fn load_image_as_base64(filepath: &str) -> String {
    let image_content: Vec<u8> = fs::read(filepath).unwrap();
    let mut enc = base64::write::EncoderStringWriter::new(base64::STANDARD);
    enc.write_all(&image_content).unwrap();

    // get the resulting String
    enc.into_inner()
}