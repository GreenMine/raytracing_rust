use crate::ray_tracer::data_structures::{Color, Vec3};
use crate::ray_tracer::material::Material;
use crate::ray_tracer::{HitInfo, Ray};

pub struct Lambertian {
    albero: Color,
}

impl Lambertian {
    pub fn new(albero: Color) -> Self {
        Self { albero }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_info: &HitInfo,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_info.normal + Vec3::random_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_info.normal;
        }
        *scattered = Ray::new(hit_info.point, scatter_direction);
        *attenuation = self.albero;

        true
    }
}
