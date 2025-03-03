use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;
use std::io;

pub fn save_image(img: &DynamicImage, path: &str) -> Result<(), io::Error> {
    let mut output = File::create(path)?;

    img.write_to(&mut output, ImageOutputFormat::Png)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}
