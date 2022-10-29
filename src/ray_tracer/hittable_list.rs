use super::Hittable;
use crate::ray_tracer::{HitInfo, Ray};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new_actual(objects: Vec<Box<dyn Hittable + 'static>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + 'static>(&mut self, value: T) {
        self.objects.push(Box::new(value));
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool {
        let mut tmp_info = HitInfo::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut tmp_info) {
                hit_anything = true;
                closest_so_far = tmp_info.t;
                *hit_info = tmp_info.clone();
            }
        }

        hit_anything
    }
}
