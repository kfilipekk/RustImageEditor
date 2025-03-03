use image::DynamicImage;

pub fn apply_filter(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}
