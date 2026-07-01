use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color) {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte: usize = (256.0 * intensity.clamp(r)) as usize;
    let gbyte: usize = (256.0 * intensity.clamp(g)) as usize;
    let bbyte: usize = (256.0 * intensity.clamp(b)) as usize;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}
