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
mod material;

use std::{f64::INFINITY, rc::Rc};
use crate::{camera::Camera, color::{Color, write_color}, hittable::HittableList, material::{Lambertian, Metal}, sphere::Sphere, vec3:: {Point3, unit_vector}};

fn main() {

    // world
    let mut world = HittableList::default();
    // let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    // let boxed_sphere = Box::new(sphere);

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,  0.0, -1.2),  0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0),  0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0,  0.0, -1.0),  0.5, material_right)));
    // world.add(gb_sphere);
    
    let cam: Camera = Camera { aspect_ratio: 16.0 / 9.0, img_width: 400, samples_per_pixel: 100, max_depth: 50 };
    
    // let g_sphere = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    // let gb_sphere = Box::new(g_sphere);

    cam.render(&world);
}
