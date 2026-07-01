// use std::io::Cursor;
// use image::ImageReader;
mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod util;
mod interval;
mod camera;

use std::f64::INFINITY;
use crate::{hittable::HittableList, color::write_color, sphere::Sphere, vec3:: {Point3, unit_vector}, camera::Camera};

fn main() {

    // world
    let mut world = HittableList::default();
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let boxed_sphere = Box::new(sphere);

    let cam: Camera = Camera { aspect_ratio: 16.0 / 9.0, img_width: 400, samples_per_pixel: 100, max_depth: 50 };
    
    let g_sphere = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    let gb_sphere = Box::new(g_sphere);
    world.add(boxed_sphere);
    world.add(gb_sphere);

    cam.render(&world);
}
