use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = read_username_from_file("hello.txt");
    match f {
        Ok(n) => println!("n: {}", n),
        Err(e) => panic!("{}", e),
    }
}