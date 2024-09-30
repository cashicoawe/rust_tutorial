fn main() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("value: {}", a[index]);
    //     index += 1;
    // }
    for element in a {
        println!("element: {}", element);
    }

    for number in (1..4).rev() {
        println!("number: {}", number);
    }
}