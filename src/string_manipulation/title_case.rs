pub fn from_pascal_to_human_title(s: &str) -> String {
    let mut str = String::new();

    for (idx, c) in s.chars().enumerate() {
        if idx == 0 {
            str.push(c)
        } else {
            if c.is_ascii_uppercase() {
                str.push(' ');
                str.push(c)
            } else {
                str.push(c)
            }
        }
    }
    str
}

pub fn from_human_title_to_pascal(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}
