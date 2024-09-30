use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    loop {
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index.trim().parse().expect("Index enterd was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is : {}",
            index, element
        );
    }
}

// ❯ cargo run --bin array_out_of_bounds
//    Compiling types v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/05.type/types)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.12s
//      Running `target/debug/array_out_of_bounds`
// Please enter an array index.
// 1
// The value of the element at index 1 is : 2
// 2
// The value of the element at index 2 is : 3
// 3
// The value of the element at index 3 is : 4
// 4
// The value of the element at index 4 is : 5
// 5
// thread 'main' panicked at src/bin/array_out_of_bounds.rs:14:23:
// index out of bounds: the len is 5 but the index is 5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace