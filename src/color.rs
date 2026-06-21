use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    let rbyte: usize = (255.999 * r) as usize;
    let gbyte: usize = (255.999 * g) as usize;
    let bbyte: usize = (255.999 * b) as usize;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
    
}
