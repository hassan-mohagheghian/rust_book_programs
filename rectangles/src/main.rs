#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 20,
    };

    let square1 = Rectangle::square(15);
    println!(
        "The area of the rectangle is {} squres pixels.",
        rect1.area()
    );

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("area of rect1 is: {}", Rectangle::area(&rect1));
    println!("area of square1 is: {}", square1.area());
}
