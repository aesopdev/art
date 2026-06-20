// #[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}


impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

fn main() {}
