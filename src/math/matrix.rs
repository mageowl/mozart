use std::{
    iter::Sum,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

use super::{point::Pt2, Radians};

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<const WIDTH: usize, const HEIGHT: usize> {
    data: [[f64; HEIGHT]; WIDTH],
}

impl<const W: usize, const H: usize> Matrix<W, H> {
    const ZERO: Self = Self {
        data: [[0.0; H]; W],
    };

    pub const fn from_rows(rows: [[f64; W]; H]) -> Self {
        let mut cols = [[0.0; H]; W];
        let mut x = 0;
        let mut y = 0;
        while x < W {
            while y < H {
                cols[x][y] = rows[y][x];
                y += 1;
            }
            x += 1;
        }

        Self { data: cols }
    }

    #[inline(always)]
    pub const fn new(cols: [[f64; H]; W]) -> Self {
        Self { data: cols }
    }

    pub fn column(&self, index: usize) -> Matrix<1, H> {
        self.data[index].into()
    }
}

impl<const W: usize, const H: usize> From<[[f64; H]; W]> for Matrix<W, H> {
    fn from(value: [[f64; H]; W]) -> Self {
        Self::new(value)
    }
}

impl<const SIZE: usize> From<[f64; SIZE]> for Matrix<1, SIZE> {
    fn from(value: [f64; SIZE]) -> Self {
        Self::new([value])
    }
}

impl From<Pt2> for Matrix<1, 2> {
    fn from(value: Pt2) -> Self {
        Self::new([value.into()])
    }
}

impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    pub const IDENTITY: Self = {
        let mut m = Self::ZERO;
        let mut i = 0;
        while i < SIZE {
            m.data[i][i] = 1.;
            i += 1;
        }

        m
    };
}

impl<const W: usize, const H: usize> Index<(usize, usize)> for Matrix<W, H> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<const W: usize, const H: usize> IndexMut<(usize, usize)> for Matrix<W, H> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<const W: usize, const H: usize> Add for Matrix<W, H> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const W: usize, const H: usize> AddAssign for Matrix<W, H> {
    fn add_assign(&mut self, rhs: Self) {
        for x in 0..W {
            for y in 0..H {
                self[(x, y)] *= rhs[(x, y)]
            }
        }
    }
}

impl<const W: usize, const H: usize> Sum for Matrix<W, H> {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut v = iter.next().unwrap_or(Self::ZERO);
        for item in iter {
            v += item;
        }
        v
    }
}

impl<const SIZE: usize> Mul<f64> for Matrix<1, SIZE> {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const SIZE: usize> MulAssign<f64> for Matrix<1, SIZE> {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..SIZE {
            self[(0, i)] *= rhs;
        }
    }
}

impl<const SIZE: usize> Mul<Matrix<SIZE, SIZE>> for Matrix<1, SIZE> {
    type Output = Self;

    fn mul(self, rhs: Matrix<SIZE, SIZE>) -> Self::Output {
        (0..SIZE).map(|i| self.column(i) * rhs[(0, i)]).sum()
    }
}

impl<const SIZE: usize> MulAssign<Matrix<SIZE, SIZE>> for Matrix<1, SIZE> {
    fn mul_assign(&mut self, rhs: Matrix<SIZE, SIZE>) {
        *self = *self * rhs;
    }
}

impl Matrix<2, 2> {
    pub fn from_rotation(rotation: Radians) -> Self {
        let sine = rotation.sin();
        let cosine = rotation.cos();

        Self::new([[cosine, sine], [-sine, cosine]])
    }
}

impl Mul<Matrix<2, 2>> for Pt2 {
    type Output = Pt2;

    fn mul(self, rhs: Matrix<2, 2>) -> Self::Output {
        (Into::<Matrix<1, 2>>::into(self) * rhs).into()
    }
}
