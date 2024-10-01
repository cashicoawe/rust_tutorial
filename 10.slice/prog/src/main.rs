fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // then, value of "word" doesn't have meaning
}

fn first_word(s: &String) -> usinze {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}