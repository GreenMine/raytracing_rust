use crate::ray_tracer::data_structures::Color;
use crate::ray_tracer::{HitInfo, Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitInfo,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
