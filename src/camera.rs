use std::io::Write;

use crate::{INFINITY, color::Color, hittable::{HitRecord, Hittable}, interval::Interval, ray::{Ray}, unit_vector, util::random_double, vec3::{Point3, Vec3}, write_color};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            // image
            aspect_ratio: 16.0 / 9.0,
            img_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}

struct CameraState {
    img_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_samples_scale: f64,
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
            eprintln!("\rScanlines remaining: {} ", s.img_height - j);
            for i in 0..self.img_width {
                // let pixel_center = s.pixel00_loc + (s.pixel_delta_u * i as f64) + (s.pixel_delta_v * j as f64);
                // let ray_direction = pixel_center - s.center;

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&mut stdout, pixel_color * s.pixel_samples_scale);
            }
        }
        eprintln!("\rDone.");
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let s = self.initialize();
        let offset = self.sample_square();
        let pixel_sample = s.pixel00_loc
                        +   ((i as f64 + offset.x()) * s.pixel_delta_u)
                        +   ((j as f64 + offset.y()) * s.pixel_delta_v);
        let ray_origin = s.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn initialize(&self) -> CameraState {
        // calculate img height and make sure it's >=1
        let img_height = std::cmp::max((self.img_width as f64 / self.aspect_ratio) as i32, 1);
        let center = Point3::new(0.0, 0.0, 0.0);
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

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

        CameraState { img_height, center, pixel_samples_scale, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let rec = &mut HitRecord::default();
        if world.hit(r, Interval::new(0.001, INFINITY), rec) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }
    
        let unit_direction: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }   
}