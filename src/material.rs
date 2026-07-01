use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, unit_vector};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

pub struct DefaultMaterial;
impl Material for DefaultMaterial {}

impl Material for Lambertian {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}