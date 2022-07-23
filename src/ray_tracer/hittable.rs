use super::{data_structures::{Point3, Vec3}, Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool;
}

pub struct HitInfo {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64
}