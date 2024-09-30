fn main() {
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("y: {}", y);

    println!("tup.0: {}", tup.0)
}