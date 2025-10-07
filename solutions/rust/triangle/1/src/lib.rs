use std::{collections::BTreeSet, ops::Add};

use num_traits::{CheckedAdd, Zero};

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Add<Output = T> + PartialOrd + Zero + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        // Assert that this is a valid triangle
        if sides.iter().all(|&x| !x.is_zero())
            && sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1]
        {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[2] == self.sides[0]
    }
}
