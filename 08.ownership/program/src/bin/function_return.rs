fn main() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("s2");
    println!("s2: {}", s2);

    let s3 = takes_and_gives_back(s2);
    // println!("s2: {}", s2);
    println!("s3: {}", s3);

    let (s4, s4_size) = return_some_value(s3);
    println!("s4: {}, s4_size: {}", s4, s4_size);
    // println!("s3: {}", s3);
}

fn gives_ownership() -> String {
    let str = String::from("given");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn return_some_value(s: String) -> (String, usize){
    let length = s.len();
    
    (s, length)
}

// rust_tutorial/08.ownership/ownership on  main [!?⇡] is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin function_return
//    Compiling ownership v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/08.ownership/ownership)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s
//      Running `target/debug/function_return`
// s1: given
// s2: s2
// s3: s2


// rust_tutorial/08.ownership/ownership on  main [!?⇡] is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin function_return
//    Compiling ownership v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/08.ownership/ownership)
// error[E0382]: borrow of moved value: `s2`
//   --> src/bin/function_return.rs:9:24
//    |
// 5  |     let s2 = String::from("s2");
//    |         -- move occurs because `s2` has type `String`, which does not implement the `Copy` trait
// ...
// 8  |     let s3 = takes_and_gives_back(s2);
//    |                                   -- value moved here
// 9  |     println!("s2: {}", s2);
//    |                        ^^ value borrowed here after move
//    |
// note: consider changing this parameter type in function `takes_and_gives_back` to borrow instead if owning the value isn't necessary
//   --> src/bin/function_return.rs:21:30
//    |
// 21 | fn takes_and_gives_back(str: String) -> String {
//    |    --------------------      ^^^^^^ this parameter takes ownership of the value
//    |    |
//    |    in this function
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//    |
// 8  |     let s3 = takes_and_gives_back(s2.clone());
//    |                                     ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` (bin "function_return") due to 1 previous error
