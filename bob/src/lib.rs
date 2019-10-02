fn is_question(q: &str) -> bool {
    q.trim().chars().last() == Some('?')
}

fn is_all_caps(q: &str) -> bool {
    let mut ret = false;
    for c in q.chars() {
        if c >= 'A' && c <= 'Z' {
            ret = true;
        }
        if c >= 'a' {
            return false;
        }
    }
    ret
}

pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    if is_question(message) {
        if is_all_caps(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else {
        if is_all_caps(message) {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
