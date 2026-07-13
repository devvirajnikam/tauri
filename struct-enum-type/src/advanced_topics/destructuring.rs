#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle { radius: u32 },
    Rectangle { width: u32, height: u32 },
}

pub fn run() {
    println!("\n10. Destructuring structs and enums");

    let point = Point { x: 5, y: 9 };
    let Point { x, y } = point;
    println!("Point parts -> x={}, y={}", x, y);

    let shapes = [
        Shape::Circle { radius: 10 },
        Shape::Rectangle {
            width: 20,
            height: 30,
        },
    ];

    for shape in shapes {
        match shape {
            Shape::Circle { radius } => println!("Circle radius: {}", radius),
            Shape::Rectangle { width, height } => {
                println!("Rectangle size: {} x {}", width, height);
            }
        }
    }
}
