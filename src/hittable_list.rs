use crate::{HitRecord, Hittable, Ray};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;
        for obj in &self.objects {
            match obj.hit(r, ray_tmin, closest_so_far) {
                Some(hr) => {
                    hit_rec = Some(hr);
                    closest_so_far = hr.t;
                }
                None => {}
            };
        }
        hit_rec
    }
}
