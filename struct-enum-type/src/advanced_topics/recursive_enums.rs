#[derive(Debug)]
enum NumberList {
    Node(i32, Box<NumberList>),
    End,
}

impl NumberList {
    fn print(&self) {
        match self {
            NumberList::Node(value, next) => {
                println!("List value: {}", value);
                next.print();
            }
            NumberList::End => println!("End of list"),
        }
    }
}

pub fn run() {
    println!("\n18. Recursive enums using Box");

    let list = NumberList::Node(
        1,
        Box::new(NumberList::Node(
            2,
            Box::new(NumberList::Node(3, Box::new(NumberList::End))),
        )),
    );

    println!("{:?}", list);
    list.print();
}
