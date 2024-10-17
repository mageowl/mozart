use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::math::Radians;

use super::matrix::Matrix;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pt2i {
    pub x: i32,
    pub y: i32,
}

#[inline(always)]
pub const fn pt2i(x: i32, y: i32) -> Pt2i {
    Pt2i { x, y }
}

impl Pt2i {
    pub const ZERO: Self = pt2i(0, 0);

    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn angle(self) -> f32 {
        f32::atan2(self.x as f32, self.y as f32)
    }

    pub fn length(self) -> f32 {
        (self.x.pow(2) as f32 + self.y.pow(2) as f32).sqrt()
    }

    pub fn length_squared(self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }

    pub fn normalized(self) -> Pt2 {
        let pt: Pt2 = self.into();
        pt.normalized()
    }
}

impl From<[i32; 2]> for Pt2i {
    fn from(value: [i32; 2]) -> Self {
        pt2i(value[0], value[1])
    }
}

impl From<(i32, i32)> for Pt2i {
    fn from(value: (i32, i32)) -> Self {
        pt2i(value.0, value.1)
    }
}

impl From<Pt2i> for [i32; 2] {
    fn from(value: Pt2i) -> Self {
        [value.x, value.y]
    }
}

impl From<Pt2i> for (i32, i32) {
    fn from(value: Pt2i) -> Self {
        (value.x, value.y)
    }
}

impl Add for Pt2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        pt2i(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pt2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pt2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        pt2i(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pt2i {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Pt2i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        pt2i(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Pt2i {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<i32> for Pt2i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        pt2i(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Pt2i {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Pt2i {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        pt2i(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for Pt2i {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<i32> for Pt2i {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        pt2i(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for Pt2i {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt2 {
    pub x: f32,
    pub y: f32,
}

#[inline(always)]
pub const fn pt2(x: f32, y: f32) -> Pt2 {
    Pt2 { x, y }
}

impl Pt2 {
    pub const ZERO: Self = pt2(0., 0.);

    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn angle(self) -> f32 {
        f32::atan2(self.x, self.y)
    }

    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn normalize(&mut self) {
        let length = self.length_squared();
        if length != 0. {
            let length = length.sqrt();
            *self /= length;
        }
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Self) -> f32 {
        self.x * other.x - self.y * other.y
    }

    pub fn sign(self) -> Pt2i {
        pt2i(self.x.signum() as i32, self.x.signum() as i32)
    }

    pub fn floor(self) -> Pt2i {
        pt2i(self.x.floor() as i32, self.y.floor() as i32)
    }

    pub fn ceil(self) -> Pt2i {
        pt2i(self.x.ceil() as i32, self.y.ceil() as i32)
    }

    pub fn round(self) -> Pt2i {
        pt2i(self.x.round() as i32, self.y.round() as i32)
    }

    pub fn rotated(self, by: Radians) -> Self {
        self * Matrix::from_rotation(by)
    }
}

impl From<Pt2i> for Pt2 {
    fn from(value: Pt2i) -> Self {
        pt2(value.x as f32, value.y as f32)
    }
}

impl From<Matrix<1, 2>> for Pt2 {
    fn from(value: Matrix<1, 2>) -> Self {
        pt2(value[(0, 0)], value[(0, 1)])
    }
}

impl From<[f32; 2]> for Pt2 {
    fn from(value: [f32; 2]) -> Self {
        pt2(value[0], value[1])
    }
}

impl From<(f32, f32)> for Pt2 {
    fn from(value: (f32, f32)) -> Self {
        pt2(value.0, value.1)
    }
}

impl From<Pt2> for [f32; 2] {
    fn from(value: Pt2) -> Self {
        [value.x, value.y]
    }
}

impl From<Pt2> for (f32, f32) {
    fn from(value: Pt2) -> Self {
        (value.x, value.y)
    }
}

impl Add for Pt2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        pt2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pt2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pt2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        pt2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pt2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Pt2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        pt2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Pt2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Pt2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        pt2(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Pt2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Pt2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        pt2(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for Pt2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f32> for Pt2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        pt2(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Pt2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
