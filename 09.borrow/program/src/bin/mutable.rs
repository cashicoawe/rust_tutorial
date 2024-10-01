fn main() {
    let mut s = String::from("s");
    println!("s: {}", s);
    change(&mut s);
    println!("s: {}", s);
}

fn change(s1: &mut String) {
    s1.push_str(" changed");
}