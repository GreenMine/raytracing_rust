use std::{
    fmt,
    ops::{AddAssign, Neg, MulAssign, Mul, Add, DivAssign, Div, Sub}
};

pub use Vec3 as Point3;
pub use Vec3 as Color;

#[derive(Copy, Clone, Debug)]
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

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 +
    u.1 * v.1 +
    u.2 * v.2
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

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self
    }
}