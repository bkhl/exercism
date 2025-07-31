pub fn encode(input: &str) -> String {
    let mut result = String::new();
    let mut iterator = input.chars();
    let mut count = 0;
    let mut last = None;

    loop {
        let curr = iterator.next();

        match curr {
            Some(curr_char) => match last {
                Some(last_char) if last_char == curr_char => count += 1,
                Some(last_char) => {
                    result += &encode_run(last_char, count);
                    count = 1
                }
                None => count += 1,
            },
            None => {
                if let Some(last_char) = last {
                    result += &encode_run(last_char, count)
                }
                break;
            }
        }

        last = curr;
    }

    result
}

fn encode_run(c: char, n: usize) -> String {
    let mut result = match n {
        0...1 => String::new(),
        _ => n.to_string(),
    };
    result.push(c);
    result
}

pub fn decode(input: &str) -> String {
    let mut result = String::new();
    let mut count = String::new();

    for curr_char in input.chars() {
        if curr_char.is_digit(10) {
            count.push(curr_char)
        } else {
            if count.is_empty() {
                result.push(curr_char)
            } else {
                for _ in 0..(count.parse().unwrap()) {
                    result.push(curr_char)
                }
                count.clear()
            }
        }
    }

    result
}