use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn compress_image(input_path: &str, output_path: &str, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();
    let mut compressed_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;
        let compressed_pixel = Rgba([
            data[0] / quality,
            data[1] / quality,
            data[2] / quality,
            data[3],
        ]);
        compressed_img.put_pixel(x, y, compressed_pixel);
    }

    compressed_img.save(output_path)?;
    Ok(())
}

fn main() {
    let input_path = "input.png";
    let output_path = "output.png";
    let quality = 2; // Image Quality Control

    if let Err(e) = compress_image(input_path, output_path, quality) {
        eprintln!("Image Compression Error: {}", e);
    }
}

