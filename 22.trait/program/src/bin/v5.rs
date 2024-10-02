use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest is x: {}", self.x);
        }else{
            println!("largest is y: {}", self.y);
        }
    }
}

fn main() {
    let p = Pair::new(1, 2);
    p.cmp_display();

    let p = Pair::new(vec![1, 2], vec![3, 4]);
    // p.cmp_display();
}