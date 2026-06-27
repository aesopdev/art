use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    let intensity = Interval::new(0.000, 0.999);
    let rbyte: usize = (256.0 * intensity.clamp(r)) as usize;
    let gbyte: usize = (256.0 * intensity.clamp(g)) as usize;
    let bbyte: usize = (256.0 * intensity.clamp(b)) as usize;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
