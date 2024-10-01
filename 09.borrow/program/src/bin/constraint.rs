fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{}", r2);

    let mut s2 = String::from("s2");
    let r1 = &s2;
    let r2 = &s2;
    // let r3 = &mut s2;

    println("r1: {}", r1);
    println("r2: {}", r2);
    // println("r3: {}", r3);
}

// ❯ cargo run --bin constraint
//    Compiling borrow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/09.borrow/borrow)
// error[E0499]: cannot borrow `s` as mutable more than once at a time
//  --> src/bin/constraint.rs:4:14
//   |
// 3 |     let r1 = &mut s;
//   |              ------ first mutable borrow occurs here
// 4 |     let r2 = &mut s;
//   |              ^^^^^^ second mutable borrow occurs here
// 5 |
// 6 |     println!("{}, {}", r1, r2);
//   |                        -- first borrow later used here

// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `borrow` (bin "constraint") due to 1 previous error


// ❯ cargo run --bin constraint
//    Compiling borrow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/09.borrow/borrow)
// error[E0502]: cannot borrow `s2` as mutable because it is also borrowed as immutable
//   --> src/bin/constraint.rs:13:14
//    |
// 11 |     let r1 = &s2;
//    |              --- immutable borrow occurs here
// 12 |     let r2 = &s2;
// 13 |     let r3 = &mut s2;
//    |              ^^^^^^^ mutable borrow occurs here
// 14 |
// 15 |     println!("{}, {}, {}", r1, r2, r3);
//    |                            -- immutable borrow later used here

// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `borrow` (bin "constraint") due to 1 previous error