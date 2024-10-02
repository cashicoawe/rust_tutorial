fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 4, 3, 2];
    println!("largest number: {:?}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {:?}", largest(&char_list));
}