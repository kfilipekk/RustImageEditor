use image::{DynamicImage, GenericImageView, Luma, imageops};
use imageproc::filter;
use imageproc::edges::sobel_gradient;

pub fn apply_filter(img: DynamicImage, filter_type: &str) -> DynamicImage {
    match filter_type {
        "grayscale" => img.to_luma8().into(),
        "sepia" => apply_sepia(&img),
        "invert" => invert_colors(&img),
        "contrast" => apply_contrast(&img, 1.5), // Example contrast factor
        "edge_detect" => sobel_gradient(&img.to_luma8()).into(),
        "sharpen" => filter::sharpen3x3(&img.to_luma8()).into(),
        _ => img,
    }
}

fn apply_sepia(img: &DynamicImage) -> DynamicImage {
    // Apply sepia filter here
    img.to_rgb8().into()
}

fn invert_colors(img: &DynamicImage) -> DynamicImage {
    img.invert().into()
}

pub fn apply_contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    use image::imageops::contrast;
    contrast(&img.to_luma8(), factor).into()
}

pub fn apply_brightness(img: &DynamicImage, value: f32) -> DynamicImage {
    use image::imageops::brightness;
    brightness(&img.to_luma8(), value).into()
}
