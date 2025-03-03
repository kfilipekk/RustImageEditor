use image::{self, DynamicImage};

pub fn load_image(path: &str) -> Result<DynamicImage, String> {
    image::open(path).map_err(|e| e.to_string())
}
