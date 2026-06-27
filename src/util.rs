pub const PI: f64 = std::f64::consts::PI;

pub fn deg_to_rads(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}