fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest: {}", result)
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

// 󰂄 10% ❯ cargo run --bin v1
//    Compiling program v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/23.lifetime/program)
// error[E0106]: missing lifetime specifier
//  --> src/bin/v1.rs:8:33
//   |
// 8 | fn longest(x: &str, y: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//   |
// 8 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `program` (bin "v1") due to 1 previous error