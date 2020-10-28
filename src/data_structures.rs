use std::{
    fmt,
    ops::{AddAssign, Neg, MulAssign, Mul, Add, DivAssign}
};

pub use Vec3 as Point3;
pub use Vec3 as Color;

#[derive(Copy, Clone)]
pub struct Vec3(
    pub(crate) f64,
    pub(crate) f64,
    pub(crate) f64
);

impl Vec3 {

    pub(crate) const fn x(&self) -> f64 {self.0}
    pub(crate) const fn y(&self) -> f64 {self.1}
    pub(crate) const fn z(&self) -> f64 {self.2}

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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
       self += rhs;
       self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs *= self;
        rhs
    }
}