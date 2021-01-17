#![allow(dead_code)]
use std::{fmt::{Display, Formatter, Result as FmtResult}, io::{Result as IoResult, Write}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn with_val(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self::with_val(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 - v.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn write_color<W: Write>(&self, w: &mut W) -> IoResult<()> {
        writeln!(
            w,
            "{} {} {}",
            (255.999 * self.0) as i32,
            (255.999 * self.1) as i32,
            (255.999 * self.2) as i32,
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0f64, 0f64, 0f64)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::with_val(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::with_val(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::with_val(-self.0, -self.1, -self.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::with_val(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::with_val(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, lhs: Vec3) -> Self::Output {
        lhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1f64 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
