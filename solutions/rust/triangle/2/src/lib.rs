extern crate num;

use num::Num;

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, &'static str> {
        let [a, b, c] = sides;

        if a.is_zero() || b.is_zero() || c.is_zero() {
            return Err("zero length side is illegal");
        }

        if a > (b + c) || b > (a + c) || c > (a + b) {
            return Err("triangle must not break triangle inequality");
        }

        Ok(Triangle(a, b, c))
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