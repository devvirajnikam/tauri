#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn run() {
    println!("\n13. Associated functions vs methods");

    let rectangle = Rectangle::new(10, 20);
    let square = Rectangle::square(5);

    println!("Rectangle: {:?}, area={}", rectangle, rectangle.area());
    println!("Square: {:?}, area={}", square, square.area());
}
