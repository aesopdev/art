// use std::io::Cursor;
// use image::ImageReader;

fn main() {
    // image
    let img_width:  i32 = 256;
    let img_height: i32 = 256;

    // render
    println!("P3\n{}{}{}{}", img_width, " ", img_height, "\n255\n");

    for j in 0..img_height {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..img_width {
            let r = (i as f64) /  (img_width as f64 - 1.0);
            let g = (j as f64) / (img_height as f64 - 1.0);
            let b = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{}{}{}{}{}{}", ir, ' ', ig, ' ', ib, '\n');
        } 
    }
    eprintln!("\rDone.");
}
