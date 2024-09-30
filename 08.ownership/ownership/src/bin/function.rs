fn main() {
    let s = String::from("string");
    takes_ownership(s);

    // println!("s: {}", s);

    let x = 5;
    makes_copy(x);

    println!("x: {}", x);
}

fn takes_ownership(str: String){
    println!("{}", str);
}

fn makes_copy(int: i32){
    println!("{}", int)
}

// rust_tutorial/08.ownership/ownership on  main [!?⇡] is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin funtion
//    Compiling ownership v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/08.ownership/ownership)
// error[E0382]: borrow of moved value: `s`
//   --> src/bin/funtion.rs:5:23
//    |
// 2  |     let s = String::from("string");
//    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
// 3  |     takes_ownership(s);
//    |                     - value moved here
// 4  |
// 5  |     println!("s: {}", s);
//    |                       ^ value borrowed here after move
//    |
// note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
//   --> src/bin/funtion.rs:13:25
//    |
// 13 | fn takes_ownership(str: String){
//    |    ---------------      ^^^^^^ this parameter takes ownership of the value
//    |    |
//    |    in this function
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 3  |     takes_ownership(s.clone());
//    |                      ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` (bin "funtion") due to 1 previous error
