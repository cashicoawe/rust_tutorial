#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size, height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {width: 100, height: 50};
    let rect2 = Rectangle {width: 50, height: 10};
    let rect3 = Rectangle {width: 40, height: 20};

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(10);
    println!("rect4: {:#?}", rect4);
}