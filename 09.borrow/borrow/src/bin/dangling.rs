fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

// ❯ cargo run --bin dangling
//    Compiling borrow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/09.borrow/borrow)
// error[E0106]: missing lifetime specifier
//  --> src/bin/dangling.rs:5:16
//   |
// 5 | fn dangle() -> &String {
//   |                ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
//   |
// 5 | fn dangle() -> &'static String {
//   |                 +++++++
// help: instead, you are more likely to want to return an owned value
//   |
// 5 - fn dangle() -> &String {
// 5 + fn dangle() -> String {
//   |

// warning: unused variable: `reference_to_nothing`
//  --> src/bin/dangling.rs:2:9
//   |
// 2 |     let reference_to_nothing = dangle();
//   |         ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_reference_to_nothing`
//   |
//   = note: `#[warn(unused_variables)]` on by default

// error[E0515]: cannot return reference to local variable `s`
//  --> src/bin/dangling.rs:7:5
//   |
// 7 |     &s
//   |     ^^ returns a reference to data owned by the current function

// Some errors have detailed explanations: E0106, E0515.
// For more information about an error, try `rustc --explain E0106`.
// warning: `borrow` (bin "dangling") generated 1 warning
// error: could not compile `borrow` (bin "dangling") due to 2 previous errors; 1 warning emitted