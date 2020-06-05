use std::fmt;

pub struct Vec3(
    pub(crate) f64,
    pub(crate) f64,
    pub(crate) f64
);

impl Vec3 {

    pub(crate) fn x(&self) -> f64 {self.0}
    pub(crate) fn y(&self) -> f64 {self.1}
    pub(crate) fn z(&self) -> f64 {self.2}

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()    
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
    } 
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
