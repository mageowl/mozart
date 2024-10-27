use std::ops::{Mul, MulAssign};

use crate::math::{matrix::Matrix, point::Pt2};

#[derive(Clone, Copy)]
pub struct Transform {
    pub(crate) mat: Matrix<2, 2>,
    pub(crate) offset: Pt2,
    pub(crate) pivot: Pt2,
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

    pub fn scaled_uniform(mut self, scale: f32) -> Self {
        self.mat += Matrix::new([[scale, 0.], [0., scale]]);
        self
    }
    pub fn scaled(mut self, scale: impl Into<Pt2>) -> Self {
        let scale = scale.into();
        self.mat += Matrix::new([[scale.x, 0.], [0., scale.y]]);
        self
    }
    pub fn with_offset(mut self, offset: impl Into<Pt2>) -> Self {
        let offset = offset.into();
        self.offset += offset;
        self
    }
    pub fn with_pivot(mut self, pivot: impl Into<Pt2>) -> Self {
        let pivot = pivot.into();
        self.pivot += pivot;
        self
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
