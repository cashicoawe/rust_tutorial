fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };
    println!("number: {}", number);
}

// rust_tutorial/07.flow/flow on  main is 📦 v0.1.0 via 🦀 v1.81.0 took 3s 
// ❯ cargo run --bin if_expression
//    Compiling flow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/07.flow/flow)
// error[E0308]: `if` and `else` have incompatible types
//  --> src/bin/if_expression.rs:4:44
//   |
// 4 |     let number = if condition { 5 } else { "six" };
//   |                                 -          ^^^^^ expected integer, found `&str`
//   |                                 |
//   |                                 expected because of this

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `flow` (bin "if_expression") due to 1 previous error
