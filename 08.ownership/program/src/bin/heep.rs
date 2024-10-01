fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
    println!("{}, world?", s1);
}

// rust_tutorial/08.ownership/ownership on  main [!?⇡] is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin heep
//    Compiling ownership v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/08.ownership/ownership)
// error[E0382]: borrow of moved value: `s1`
//  --> src/bin/heep.rs:6:28
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// ...
// 6 |     println!("{}, world?", s1);
//   |                            ^^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     let s2 = s1.clone();
//   |                ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` (bin "heep") due to 1 previous error
