// #[derive(Debug)]

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("Vec::new() -> {:?}", v);

    let v = vec![1, 2, 3];
    println!("vec![1, 2, 3] -> {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v.push() 5, 6, 7 -> {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third: {}", third);

    match v.get(2) {
        Some(third) => println!("v.get(2): {}", third),
        Node => println!("none"),
    }
}
