fn main() {
    let mut s = String::from("s");
    println!("s: {}", s);

    change(&s);

    println!("s: {}", s);
}

fn change(s1: &String) {
    s1.push_str(" changed");
}

// ❯ cargo run --bin immutable
//    Compiling borrow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/08.ownership/borrow)
// error[E0596]: cannot borrow `*s1` as mutable, as it is behind a `&` reference
//   --> src/bin/immutable.rs:11:5
//    |
// 11 |     s1.push_str(" changed");
//    |     ^^ `s1` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//    |
// help: consider changing this to be a mutable reference
//    |
// 10 | fn change(s1: &mut String) {
//    |                +++

// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `borrow` (bin "immutable") due to 1 previous error
