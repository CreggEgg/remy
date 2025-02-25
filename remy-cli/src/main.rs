use std::io::{BufWriter, Cursor};

use image::io::Reader as ImageReader;
use rascii_art::{render_image, render_to, RenderOptions};

fn main() {
    let mut buffer = Vec::new();
    let image = ImageReader::new(Cursor::new(include_bytes!("../assets/remy.jpg")))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    render_image(
        &image,
        &mut buffer,
        &RenderOptions::new()
            .height(32)
            .invert(true)
            .colored(true)
            .charset(&[" ", /*"░",*/ " ", "▒", "▓", "█"]),
    )
    .unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));
}
