fn match_brac(a: char, b: char) -> bool {
    match a {
        '(' => b == ')',
        '[' => b == ']',
        '{' => b == '}',
        '<' => b == '>',
        _ => false,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for c in string.chars() {
        match c {
            '(' | '[' | '{' | '<' => brackets.push(c),
            ')' | ']' | '}' | '>' => match brackets.last() {
                Some(k) if match_brac(*k, c) => { brackets.pop(); },
                _ => return false,
            },
            _ => {},
        }
    }

    brackets.is_empty()
}
