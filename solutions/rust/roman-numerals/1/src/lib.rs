use std::fmt;

extern crate num;
use num::{Integer, ToPrimitive, Unsigned};

#[derive(Clone, Copy)]
enum RomanDigit {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

pub struct Roman(Vec<RomanDigit>);

impl<T> From<T> for Roman
where
    T: Integer + Unsigned + ToPrimitive,
{
    fn from(n: T) -> Self {
        let mut v = Vec::<RomanDigit>::new();
        let i = n.to_usize().unwrap();

        let i = from_partial(RomanDigit::M, RomanDigit::D, RomanDigit::C, &mut v, i);
        let i = from_partial(RomanDigit::C, RomanDigit::L, RomanDigit::X, &mut v, i);
        from_partial(RomanDigit::X, RomanDigit::V, RomanDigit::I, &mut v, i);

        Roman(v)
    }
}

fn from_partial(
    high: RomanDigit,
    mid: RomanDigit,
    low: RomanDigit,
    numeral: &mut Vec<RomanDigit>,
    mut remainder: usize,
) -> usize {
    while remainder >= high as usize {
        numeral.push(high);
        remainder -= high as usize;
    }

    if remainder >= high as usize - low as usize {
        numeral.push(low);
        numeral.push(high);
        remainder -= high as usize - low as usize;
    }

    if remainder >= mid as usize {
        numeral.push(mid);
        remainder -= mid as usize;
    }

    if remainder >= mid as usize - low as usize {
        numeral.push(low);
        numeral.push(mid);
        remainder -= mid as usize - low as usize;
    }

    while remainder >= low as usize {
        numeral.push(low);
        remainder -= low as usize;
    }

    remainder
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Roman(digits) = self;

        digits
            .iter()
            .map(|digit| match digit {
                RomanDigit::I => 'I',
                RomanDigit::V => 'V',
                RomanDigit::X => 'X',
                RomanDigit::L => 'L',
                RomanDigit::C => 'C',
                RomanDigit::D => 'D',
                RomanDigit::M => 'M',
            })
            .collect::<String>()
            .fmt(f)
    }
}