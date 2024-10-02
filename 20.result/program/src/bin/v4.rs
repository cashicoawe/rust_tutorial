use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    println!("{}", filename);
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), 
    }
}

fn main() {
    let f = read_username_from_file("hello.txt");
    match f {
        Ok(name) => println!("name: {:?}", name),
        Err(e) => panic!("error: {:?}", e),
    }
}