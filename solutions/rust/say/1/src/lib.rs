fn word(n: u64) -> String {
    let s = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        1_000 => "thousand",
        1_000_000 => "million",
        1_000_000_000 => "billion",
        1_000_000_000_000 => "trillion",
        1_000_000_000_000_000 => "quadrillion",
        1_000_000_000_000_000_000 => "quintillion",
        _ => panic!(),
    };
    String::from(s)
}

pub fn encode(n: u64) -> String {
    match n {
        0 => word(0),
        _ => encode_nonzero(n),
    }
}

fn encode_nonzero(n: u64) -> String {
    let mut strings = Vec::new();

    let mut d = 1_000_000_000_000_000_000;
    let mut r = n;

    while d > 1 {
        let q = r / d;
        r = r % d;

        if q > 0 {
            strings.push(encode_hundreds(q));
            strings.push(word(d));
        }

        d /= 1000;
    }

    if r > 0 {
        strings.push(encode_hundreds(r));
    }

    strings.join(" ")
}

fn encode_hundreds(n: u64) -> String {
    let mut strings = Vec::new();

    let q = n / 100;
    let r = n % 100;

    if q > 0 {
        strings.push(word(q));
        strings.push(word(100));
    }
    if r > 0 {
        strings.push(encode_tens(r));
    }

    strings.join(" ")
}

fn encode_tens(n: u64) -> String {
    let mut strings = Vec::new();

    if n > 20 {
        let q = n / 10;
        let r = n % 10;

        if q > 0 {
            strings.push(word(q * 10));
        }

        if r > 0 {
            strings.push(word(r));
        }
    } else {
        strings.push(word(n));
    }

    strings.join("-")
}