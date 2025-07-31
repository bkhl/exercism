pub fn verse(n: i32) -> String {
    match n {
        0 => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
                Take it down and pass it around, no more bottles of beer on the wall.\n",
        ),
        n => {
            format!(
                "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottle{2} of beer on the wall.\n",
                n,
                n - 1,
                if n - 1 > 1 { "s" } else { "" }
            )
        }
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut s = String::new();
    for n in ((end + 1)..(start + 1)).rev() {
        s += &verse(n);
        s.push('\n');
    }
    s += &verse(end);
    s
}