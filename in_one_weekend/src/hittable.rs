use crate::{Interval, Material, Point3, Ray, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(r: Ray, t: f64, p: Point3, outward_normal: Vec3, mat: Rc<dyn Material>) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;
}
