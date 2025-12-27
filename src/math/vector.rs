// standard imports
use std::fmt;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};

type Scalar = f64;

#[derive(Debug)]
pub struct Vector <const DIM: usize> {
    data : [Scalar; DIM],
}


impl<const DIM:usize> Vector<DIM> {
    
    pub fn dot(&self, other : Self) -> Scalar {
        let mut acc : Scalar = 0.0;
        for i in 0..DIM {
            acc += self.data[i] * other.data[i];
        }
        acc
    }
}

type Vector2f = Vector<2>;
type Vector3f = Vector<3>;
type Vector4f = Vector<4>;


impl<const DIM: usize> Add for Vector<DIM> {
    type Output = Vector<DIM>;
    
    fn add(self, other : Self) -> Self::Output {
        let mut data : [Scalar; DIM] = self.data;
        for i in 0..DIM {
            data[i] += other.data[i];
        }
        Self::Output {data}
    }
    
}


impl<const DIM: usize> AddAssign for Vector<DIM> {

    fn add_assign(&mut self, other : Self) {
        for i in 0..DIM {
            self.data[i] += other.data[i];
        }
    }
    
}

impl<const DIM: usize> Sub for Vector<DIM> {
    type Output = Vector<DIM>;
    
    fn sub(self, other : Self) -> Self::Output {
        let mut data : [Scalar; DIM] = self.data;
        for i in 0..DIM {
            data[i] -= other.data[i];
        }
        Self::Output {data}
    }
    
}

impl<const DIM: usize> SubAssign for Vector<DIM> {

    fn sub_assign(&mut self, other : Self) {
        for i in 0..DIM {
            self.data[i] -= other.data[i];
        }
    }
    
}




