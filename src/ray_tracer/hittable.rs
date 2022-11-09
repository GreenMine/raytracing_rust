use super::{
    data_structures::{Point3, Vec3},
    Ray,
};
use crate::ray_tracer::material::Material;
use crate::ray_tracer::materials;
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool;
}

pub struct HitInfo {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitInfo {
    fn default() -> Self {
        Self {
            material: Arc::new(materials::Default),
            ..Default::default()
        }
    }
}

impl HitInfo {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face =
            crate::ray_tracer::data_structures::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
