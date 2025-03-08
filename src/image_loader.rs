use image::{DynamicImage, io::Reader as ImageReader};
use std::fs::File;
use std::path::Path;

pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}
