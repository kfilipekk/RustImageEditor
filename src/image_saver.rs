use image::DynamicImage;
use std::fs::File;
use std::path::Path;
use std::error::Error;

pub fn save_image(img: &DynamicImage, path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    img.save(path).map_err(|e| Box::new(e) as Box<dyn Error>)
}