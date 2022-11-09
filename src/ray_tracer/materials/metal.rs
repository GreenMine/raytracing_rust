use crate::ray_tracer::data_structures::{dot, reflect, unit_vector, Color, Vec3};
use crate::ray_tracer::material::Material;
use crate::ray_tracer::{HitInfo, Ray};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitInfo,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
        *scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        dot(&scattered.direction, &hit_record.normal) > 0.0
    }
}
