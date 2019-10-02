pub fn abbreviate(phrase: &str) -> String {
    let mut prev;
    let mut current = ' ';
    let mut acc = String::new();
    for c in phrase.chars() {
        prev = current;
        current = c;
        match prev {
            'a'..='z' => if current.is_ascii_uppercase() {
                acc.push(current.to_ascii_uppercase());
            },
            ' ' | '-' => acc.push(current.to_ascii_uppercase()),
            _ => {},
        }
    }

    acc
}
