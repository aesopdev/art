// use std::io::Cursor;
// use image::ImageReader;
mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::ray::Ray;
use crate::color::write_color;
use crate::color::Color;
use crate::vec3::Point3;
use crate::vec3::dot;
use crate::vec3::unit_vector;
use vec3::Vec3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        // return -b - discriminant.sqrt() / 2.0 * a; TODO: try this at end instead of below line
        // return h - discriminant.sqrt() / a; SAME AS ABOVE
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction: Vec3 = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_width: i32 = 400;

    // calculate img height and make sure it's >=1
    let img_height = ((img_width as f64) / aspect_ratio) as i32;
    let img_height = std::cmp::max(img_height, 1);

    let focal_length: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((img_width as f64) / (img_height as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // calculate vectors across the horizontal and down vertical viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate horizontal and vertical delta vectors
    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    // location of upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    
    let mut stdout = std::io::stdout().lock();

    // render
    println!("P3\n{} {}\n255", img_width, img_height);

    for j in 0..img_height {
        // eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..img_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut stdout, pixel_color);
        }
    }
    eprintln!("\rDone.");
}
