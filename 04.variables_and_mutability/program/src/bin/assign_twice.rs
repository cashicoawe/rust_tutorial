fn main() {
    let x = 5;
    println!("x: {}", x);
    x = 10;
    println!("x: {}", x);
}

// rust_tutorial/04.variables_and_mutability/variables on  main is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin assign_twice   
//    Compiling variables v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/04.variables_and_mutability/variables)
// error[E0384]: cannot assign twice to immutable variable `x`
//  --> src/bin/assign_twice.rs:4:5
//   |
// 2 |     let x = 5;
//   |         - first assignment to `x`
// 3 |     println!("x: {}", x);
// 4 |     x = 10;
//   |     ^^^^^^ cannot assign twice to immutable variable
//   |
// help: consider making this binding mutable
//   |
// 2 |     let mut x = 5;
//   |         +++

// For more information about this error, try `rustc --explain E0384`.
// error: could not compile `variables` (bin "assign_twice") due to 1 previous error