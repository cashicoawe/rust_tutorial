fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // it is same result with &s[..5]
    let world = &s[6..11];
}

fn first_word(s: &String) -> &str { // &str means string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..1];
        }
    }

    &s[..]
}