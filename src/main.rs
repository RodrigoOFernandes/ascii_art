use image::{ImageReader, GenericImageView};
use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter the path to the image: ");
    stdout().flush()?;

    let mut img_path = String::new();
    stdin().read_line(&mut img_path)?;

    let img_path = img_path.trim();

    let img = ImageReader::open(img_path)?.decode()?;

    let grayscale = img.grayscale();

    let gradient = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

    let (width, height) = grayscale.dimensions();
    let downsample_factor_full = 2; // Adjust for full-resolution detail
    let mut full_ascii = String::new();

    for y in (0..height).step_by(downsample_factor_full as usize) {
        for x in (0..width).step_by(downsample_factor_full as usize) {
            let pixel = grayscale.get_pixel(x, y);
            let intensity = pixel[0];
            let index = (intensity as f32 / 255.0 * (gradient.len() - 1) as f32) as usize;
            let char = gradient.chars().nth(index).unwrap();
            full_ascii.push(char);
        }
        full_ascii.push('\n');
    }

    let downsample_factor_minimap = 10; // Adjust for mini-map size
    let mut minimap_ascii = String::new();

    for y in (0..height).step_by(downsample_factor_minimap as usize) {
        for x in (0..width).step_by(downsample_factor_minimap as usize) {
            let pixel = grayscale.get_pixel(x, y);
            let intensity = pixel[0];
            let index = (intensity as f32 / 255.0 * (gradient.len() - 1) as f32) as usize;
            let char = gradient.chars().nth(index).unwrap();
            minimap_ascii.push(char);
        }
        minimap_ascii.push('\n');
    }

    let full_file = File::create("resultado_full.txt")?;
    let mut full_writer = BufWriter::new(full_file);
    write!(full_writer, "{}", full_ascii)?;
    println!("Full-resolution ASCII art saved as 'resultado_full.txt'!");

    let minimap_file = File::create("resultado_minimap.txt")?;
    let mut minimap_writer = BufWriter::new(minimap_file);
    write!(minimap_writer, "{}", minimap_ascii)?;
    println!("Mini-map ASCII art saved as 'resultado_minimap.txt'!");

    Ok(())
}
