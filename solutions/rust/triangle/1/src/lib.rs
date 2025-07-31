extern crate num;

use num::Num;

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn build(mut sides: [T; 3]) -> Result<Triangle<T>, &'static str> {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sides[0].is_zero() {
            return Err("zero length side is illegal");
        }

        if sides[2] > (sides[0] + sides[1]) {
            return Err("triangle must not break triangle inequality");
        }

        Ok(Triangle(sides[0], sides[1], sides[2]))
    }

    pub fn is_equilateral(&self) -> bool {
        let Triangle(a, b, c) = *self;

        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        let Triangle(a, b, c) = *self;

        (a != b) && (a != c) && (b != a) && (b != c)
    }

    pub fn is_isosceles(&self) -> bool {
        let Triangle(a, b, c) = *self;

        a == b || a == c || b == c
    }
}