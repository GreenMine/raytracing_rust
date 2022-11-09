use super::Hittable;
use crate::ray_tracer::{HitInfo, Ray};
use std::marker::PhantomData;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable>>,
    phantom: PhantomData<&'a ()>,
}

unsafe impl<'a> Sync for HittableList<'a> {}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn new_actual(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            objects,
            phantom: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hittable + 'static>(&mut self, value: T) {
        self.objects.push(Box::new(value));
    }

    pub fn hit<'b>(
        &'b self,
        ray: &'b Ray,
        t_min: f64,
        t_max: f64,
        hit_info: &'b mut HitInfo<'b>,
    ) -> bool {
        let mut tmp_info = HitInfo::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|obj| {
            if obj.hit(ray, t_min, closest_so_far, &mut tmp_info) {
                hit_anything = true;
                closest_so_far = tmp_info.t;
            }
        });

        if hit_anything {
            *hit_info = tmp_info;
        }

        hit_anything
    }
}
