use crate::math::real::{Real, PI};
// standard imports

use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    pub real: Real,
    pub i: Real,
    pub j: Real,
    pub k: Real,
}

impl Quaternion {
    pub fn from_real(r: Real) -> Self {
        Self {
            real: r,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

    pub fn i() -> Self {
        Self {
            real: 0.0,
            i: 1.0,
            j: 0.0,
            k: 0.0,
        }
    }

    pub fn j() -> Self {
        Self {
            real: 0.0,
            i: 0.0,
            j: 1.0,
            k: 0.0,
        }
    }

    pub fn k() -> Self {
        Self {
            real: 0.0,
            i: 0.0,
            j: 0.0,
            k: 1.0,
        }
    }

    pub fn conj(&self) -> Self {
        Self {
            real: self.real,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    pub fn conjugate(&self) -> Self {
        self.conj()
    }

    pub fn abs_sq(&self) -> Real {
        self.real.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)
    }

    pub fn abs(&self) -> Real {
        self.abs_sq().sqrt()
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0} + {1} i + {2} j + {3} k",
            self.real, self.i, self.j, self.k
        )
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.i * other.i - self.j * other.j - self.k * other.k;
        let i = self.real * other.i + self.i * other.real + self.j * other.k - self.k * other.j;
        let j = self.real * other.j - self.i * other.k + self.j * other.real + self.k * other.i;
        let k = self.real * other.k + self.i * other.j - self.j * other.i + self.k * other.real;
        Self { real, i, j, k }
    }
}
