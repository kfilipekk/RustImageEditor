use image::DynamicImage;
use std::fs::File;
use std::path::Path;

pub fn save_image(img: &DynamicImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}
