fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
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
    let result = largest_i32(&number_list);
    println!("largest i32: {}", result);

    let char_list= vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("largest char: {}", result);
}