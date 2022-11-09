use super::{
    data_structures::{Point3, Vec3},
    Ray,
};
use crate::ray_tracer::material::Material;
use crate::ray_tracer::materials;
use std::sync::Arc;

pub trait Hittable {
    fn hit<'a>(&'a self, ray: &'a Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo<'a>) -> bool;
}

pub struct HitInfo<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<&'a dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> Default for HitInfo<'a> {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> HitInfo<'a> {
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
