// use std::io::Cursor;
// use image::ImageReader;
mod color;
mod vec3;
mod ray;

use color::write_color;
use vec3::Vec3;

fn main() {
    // image
    let img_width: i32 = 256;
    let img_height: i32 = 256;
    let mut stdout = std::io::stdout().lock();

    // render
    println!("P3\n{} {}\n255", img_width, img_height);

    for j in 0..img_height {
        // eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..img_width {
            let pixel_color = Vec3::new(
                i as f64 / (img_width - 1) as f64,
                j as f64 / (img_height - 1) as f64,
                0.0,
            );
            write_color(&mut stdout, pixel_color);
        }
    }
    eprintln!("\rDone.");
}
