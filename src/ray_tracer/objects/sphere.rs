pub use crate::ray_tracer::{
    data_structures::{dot, Point3},
    HitInfo, Hittable, Ray,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool {
        let pos = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = dot(&ray.direction, &pos);
        let c = pos.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = discriminant.sqrt();
        let mut x = (-half_b - sqrt_disc) / (2f64 * a);
        if x < t_min || t_max < x {
            x = (-half_b + sqrt_disc) / (2f64 * a);
            if x < t_min || t_max < x {
                return false;
            }
        }

        hit_info.t = x;
        hit_info.point = ray.at(x);

        let outward_normal = (ray.direction - self.center) / self.radius;
        hit_info.set_face_normal(&ray, outward_normal);

        true
    }
}
