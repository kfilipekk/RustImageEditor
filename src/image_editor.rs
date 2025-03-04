use image::{DynamicImage, GenericImageView, Rgba}; // Added missing trait import

pub fn apply_filter(img: DynamicImage, filter_type: &str) -> DynamicImage {
    match filter_type {
        "grayscale" => img.grayscale(),
        "sepia" => apply_sepia(img),
        "invert" => apply_invert(img),
        "contrast" => apply_contrast(img),
        _ => img, // Default: return original image if filter type is unknown
    }
}

fn apply_sepia(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rgba_image = img.to_rgba8();

    for y in 0..height {
        for x in 0..width {
            let px = rgba_image.get_pixel_mut(x, y);
            let [r, g, b, a] = px.0; // Correctly extract pixel values into an array

            let tr = ((0.393 * r as f32 + 0.769 * g as f32 + 0.189 * b as f32).min(255.0)) as u8;
            let tg = ((0.349 * r as f32 + 0.686 * g as f32 + 0.168 * b as f32).min(255.0)) as u8;
            let tb = ((0.272 * r as f32 + 0.534 * g as f32 + 0.131 * b as f32).min(255.0)) as u8;

            *px = Rgba([tr, tg, tb, a]); // Corrected tuple issue
        }
    }
    DynamicImage::ImageRgba8(rgba_image)
}

fn apply_invert(img: DynamicImage) -> DynamicImage {
    let mut rgba_image = img.to_rgba8();
    for px in rgba_image.pixels_mut() {
        px.0[0] = 255 - px.0[0]; // Invert Red
        px.0[1] = 255 - px.0[1]; // Invert Green
        px.0[2] = 255 - px.0[2]; // Invert Blue
    }
    DynamicImage::ImageRgba8(rgba_image)
}

fn apply_contrast(img: DynamicImage) -> DynamicImage {
    let mut rgba_image = img.to_rgba8();
    let factor: f32 = 1.5;

    for px in rgba_image.pixels_mut() {
        px.0[0] = ((px.0[0] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
        px.0[1] = ((px.0[1] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
        px.0[2] = ((px.0[2] as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
    }

    DynamicImage::ImageRgba8(rgba_image)
}
