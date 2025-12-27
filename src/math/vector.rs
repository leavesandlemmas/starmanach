use crate::math::real::{Real, PI};
// standard imports

use std::fmt;
use std::ops;


type Vec2 = Vec<2>;
type Vec3 = Vec<3>;

pub struct Vec<const DIM: usize> {
    arr: [Real; DIM],
}


impl<const DIM: usize> Vec<{DIM}> {
    
    pub fn iter(&self) -> impl Iterator {
        self.arr.iter()
    }
    

//    pub fn from_binary(f : impl Fn(Real, Real) -> Real, u : Self, v : Self) -> Self {
//        let mut arr : [Real, DIM];
//        for i in 0..DIM {
//            *arr[i] = f(u.arr[i], v.arr[i]); 
//        }        
//        Self {arr}
//    }

}

impl<const DIM: usize> ops::Index<usize> for Vec<{DIM}> {
    type Output = Real;

    fn index(&self, i: usize) -> &Self::Output {
        &self.arr[i]
    }
}


impl<const DIM: usize> ops::IndexMut<usize> for Vec<{DIM}> {
    type Output = Real;

    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.arr[i]
    }
}

//impl<const DIM: usize> ops::Add for Vec<{DIM}> {
//    type Output = Self;
//
//    fn add(self, other: Self) -> Self {
//        let pairs =  self.arr.iter().zip(other.arr.iter());
//        let mut arr: [Real; DIM];
//        for (i, val) in pairs.map(|(x, y)| x + y).enumerate() {
//            arr[i] = val;
//        }
//        Self { arr }
//    }
//}
