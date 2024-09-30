fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let a = [3; 5];
    println!("a: {:?}", a);

    let first = a[0];
    let second = a[1];

    println!("first: {}, second: {}", first, second);
}