

use crate::{interval::Interval, ray::{Ray}, vec3::{Point3, Vec3, dot}};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3, 
    pub normal: Vec3, 
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // sets hit record normal vec.
        // outward_normal has unit length.
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: Vec::new() }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut closest_so_far = ray_t.max;
        let mut hit_anything: bool = false;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut list = Self { objects: Vec::new() };
        list.add(object);
        list
    }
    
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}