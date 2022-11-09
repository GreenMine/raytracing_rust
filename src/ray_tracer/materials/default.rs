use crate::ray_tracer::data_structures::{Color, Vec3};
use crate::ray_tracer::material::Material;
use crate::ray_tracer::{HitInfo, Ray};

#[derive(Default)]
pub struct Default;

impl Material for Default {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitInfo,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        unreachable!()
    }
}
