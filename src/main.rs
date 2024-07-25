mod pixel_hash;
use std::{collections::HashMap, env};

use image::{GenericImageView, ImageBuffer, Pixel, Rgb};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Not enough arguments: cq <image> <depth>");
    }

    let image = &args[1];
    let depth: i8 = args[2].parse().unwrap();

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Done ✅");
    println!("Image dimensions: {} - {}", img.width(), img.height());

    let mut tree: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();

    for (x, y, pixel) in img.pixels() {
        let pixel_rgb = pixel.to_rgb();
        let rgb = pixel_rgb.channels();
        let pixel_hash = pixel_hash::get_pixel_hash(rgb, depth);

        if tree.contains_key(&pixel_hash) {
            let v = tree.get(&pixel_hash).unwrap();
            tree.insert(
                pixel_hash,
                (
                    v.0 + 1,
                    v.1 + u64::from(pixel_rgb[0]),
                    v.2 + u64::from(pixel_rgb[1]),
                    v.3 + u64::from(pixel_rgb[2]),
                ),
            );
        } else {
            tree.insert(
                pixel_hash,
                (
                    1,
                    pixel_rgb[0].into(),
                    pixel_rgb[1].into(),
                    pixel_rgb[2].into(),
                ),
            );
        }
    }

    println!("Palette colors: {}", tree.len());

    let output_path = format!("{}/{}", current_dir, "output.png");
    let mut output = ImageBuffer::new(img.width(), img.height());
    for (x, y, pixel) in img.pixels() {
        let pixel_rgb = pixel.to_rgb();
        let rgb = pixel_rgb.channels();
        let pixel_hash = pixel_hash::get_pixel_hash(rgb, depth);
        let pixel_data = tree.get(&pixel_hash).unwrap();
        let pixel_quantized_color: [u8; 3] = [
            (pixel_data.1 / pixel_data.0) as u8,
            (pixel_data.2 / pixel_data.0) as u8,
            (pixel_data.3 / pixel_data.0) as u8,
        ];
        let new_pixel = Rgb(pixel_quantized_color);

        output.put_pixel(x, y, new_pixel);
    }

    output.save(output_path).unwrap();
}
