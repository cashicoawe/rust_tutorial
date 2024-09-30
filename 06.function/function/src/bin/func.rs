fn main() {
    println!("Hello, world!");
    another_function();
    sig_fn(5, 'h');
    block(10);

    let x = five();
    println!("x: {}", x);
}

fn another_function() {
    println!("Another function");
}

fn sig_fn(x: i32, label: char){
    println!("x: {}, label: {}", x, label);
}

fn block(x: i32){
    let a = {
        let b = x;
        b + 1
    };

    println!("a: {}", a);
}

fn five() -> i32 {
    5
}