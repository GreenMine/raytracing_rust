use super::{
    Vec3, Point3
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    
    pub fn at(&mut self, t: f64)  -> Point3 {
        self.origin + t * self.direction
    }
}