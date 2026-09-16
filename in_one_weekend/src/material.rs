use rand::Rng;

use crate::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord, _rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        None
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector(rng));
        let scattered = Ray::new(rec.p, reflected);
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
