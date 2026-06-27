use std::io::Write;

use crate::{write_color, interval::Interval, ray::{Ray}, vec3::{Point3, Vec3}, hittable::{Hittable, HitRecord}, color::Color, unit_vector, INFINITY};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            // image
            aspect_ratio: 16.0 / 9.0,
            img_width: 400,
        }
    }
}

struct CameraState {
    img_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    
    pub fn render(&self, world: &impl Hittable) {
        let s = self.initialize();
        let mut stdout = std::io::stdout().lock();

        // render
        writeln!(stdout, "P3\n{} {}\n255", self.img_width, s.img_height).unwrap();

        for j in 0..s.img_height {
            // eprintln!("\rScanlines remaining: {} ", j);
            for i in 0..self.img_width {
                let pixel_center = s.pixel00_loc + (s.pixel_delta_u * i as f64) + (s.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - s.center;
                let r = Ray::new(s.center, ray_direction);
    
                let pixel_color = Self::ray_color(&r, world);
                write_color(&mut stdout, pixel_color);
            }
        }
        eprintln!("\rDone.");
    }

    fn initialize(&self) -> CameraState {
        // calculate img height and make sure it's >=1
        let img_height = std::cmp::max((self.img_width as f64 / self.aspect_ratio) as i32, 1);
        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length: f64 = 1.0;
        let viewport_height = 2.0;

        let viewport_width = viewport_height * (self.img_width as f64 / img_height as f64);

        // calculate vectors across the horizontal and down vertical viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate horizontal and vertical delta vectors
        let pixel_delta_u = viewport_u / self.img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;

        // location of upper left pixel
        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        CameraState { img_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let rec = &mut HitRecord::default();
        if world.hit(r, Interval::new(0.0, INFINITY), rec) {
            return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
        }
    
        let unit_direction: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }   
}