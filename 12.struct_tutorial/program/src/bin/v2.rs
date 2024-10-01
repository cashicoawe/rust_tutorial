fn main() {
    let rect1 = (30, 50);
    println!("area: {}", area(rect1));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}