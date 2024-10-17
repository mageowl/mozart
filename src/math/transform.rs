use std::ops::{Mul, MulAssign};

use crate::math::{matrix::Matrix, point::Pt2};

#[derive(Clone, Copy)]
pub struct Transform {
    mat: Matrix<2, 2>,
    offset: Pt2,
    pivot: Pt2,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        mat: Matrix::IDENTITY,
        offset: Pt2::ZERO,
        pivot: Pt2::ZERO,
    };

    pub fn new(mat: Matrix<2, 2>, offset: Pt2, pivot: Pt2) -> Self {
        Self { mat, offset, pivot }
    }
}

impl Mul<Transform> for Pt2 {
    type Output = Pt2;

    fn mul(self, rhs: Transform) -> Self::Output {
        (self - rhs.pivot) * rhs.mat + rhs.offset
    }
}

impl MulAssign<Transform> for Pt2 {
    fn mul_assign(&mut self, rhs: Transform) {
        *self -= rhs.pivot;
        *self *= rhs.mat;
        *self += rhs.offset;
    }
}
