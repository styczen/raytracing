use crate::{HitRecord, Hittable, Interval, Ray};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest = None;
        for obj in &self.objects {
            let max = closest.as_ref().map_or(ray_t.max, |hr: &HitRecord| hr.t);
            if let Some(hr) = obj.hit(r, Interval::new(ray_t.min, max)) {
                closest = Some(hr);
            }
        }
        closest
    }
}
