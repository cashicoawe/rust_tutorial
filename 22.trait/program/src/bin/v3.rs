// fn largest<T>(list: &[T]) -> T {
fn largest<T: PartialOrd>(list: &[T]) -> T{
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

// rust_tutorial/22.trait/program on  main is 📦 v0.1.0 via 🦀 v1.81.0 
// ❯ cargo run --bin largest
//    Compiling program v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/22.trait/program)
// error[E0508]: cannot move out of type `[T]`, a non-copy slice
//  --> src/bin/largest.rs:3:23
//   |
// 3 |     let mut largest = list[0];
//   |                       ^^^^^^^
//   |                       |
//   |                       cannot move out of here
//   |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
//   |
// help: if `T` implemented `Clone`, you could clone the value
//  --> src/bin/largest.rs:2:12
//   |
// 2 | fn largest<T: PartialOrd>(list: &[T]) -> T{
//   |            ^ consider constraining this type parameter with `Clone`
// 3 |     let mut largest = list[0];
//   |                       ------- you could clone this value
// help: consider borrowing here
//   |
// 3 |     let mut largest = &list[0];
//   |                       +

// error[E0507]: cannot move out of a shared reference
//  --> src/bin/largest.rs:4:18
//   |
// 4 |     for &item in list.iter() {
//   |          ----    ^^^^^^^^^^^
//   |          |
//   |          data moved here
//   |          move occurs because `item` has type `T`, which does not implement the `Copy` trait
//   |
// help: consider removing the borrow
//   |
// 4 -     for &item in list.iter() {
// 4 +     for item in list.iter() {
//   |

// Some errors have detailed explanations: E0507, E0508.
// For more information about an error, try `rustc --explain E0507`.
// error: could not compile `program` (bin "largest") due to 2 previous errors
