fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { 
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 5, 0];
    let result = largest(&number_list);
    println!("largest i32: {}", result);

    let char_list= vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("largest char: {}", result);
}

// rust_tutorial/21.generics/program on  main is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin v2
//    Compiling program v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/21.generics/program)
// error[E0369]: binary operation `>` cannot be applied to type `T`
//  --> src/bin/v2.rs:4:17
//   |
// 4 |         if item > largest { 
//   |            ---- ^ ------- T
//   |            |
//   |            T
//   |
// help: consider restricting type parameter `T`
//   |
// 1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//   |             ++++++++++++++++++++++

// For more information about this error, try `rustc --explain E0369`.
// error: could not compile `program` (bin "v2") due to 1 previous error
