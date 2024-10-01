fn main() {
    let string = String::from("string is string");
    let str = "str is not string";

    let s = first_word(&string[..]);
    println!("s: {}", s);

    let s = first_word(&str);
    println!("s: {}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}