fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z, a) = tup;
    println!("y: {}", y)
}

// ❯ cargo run --bin tup_unmatch
//    Compiling types v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/05.type/types)
// error[E0308]: mismatched types
//  --> src/bin/tup_unmatch.rs:3:9
//   |
// 3 |     let (x, y) = tup;
//   |         ^^^^^^   --- this expression has type `(i32, f64, u8)`
//   |         |
//   |         expected a tuple with 3 elements, found one with 2 elements
//   |
//   = note: expected tuple `(i32, f64, u8)`
//              found tuple `(_, _)`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `types` (bin "tup_unmatch") due to 1 previous error





// rust_tutorial/05.type/types on  main is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin tup_unmatch
//    Compiling types v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/05.type/types)
// error[E0308]: mismatched types
//  --> src/bin/tup_unmatch.rs:3:9
//   |
// 3 |     let (x, y, z, a) = tup;
//   |         ^^^^^^^^^^^^   --- this expression has type `(i32, f64, u8)`
//   |         |
//   |         expected a tuple with 3 elements, found one with 4 elements
//   |
//   = note: expected tuple `(i32, f64, u8)`
//              found tuple `(_, _, _, _)`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `types` (bin "tup_unmatch") due to 1 previous error
