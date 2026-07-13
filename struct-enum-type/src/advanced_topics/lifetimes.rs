#[derive(Debug)]
struct NameView<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> NameView<'a> {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run() {
    println!("\n16. Lifetimes inside structs");

    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let name_view = NameView {
        first_name: &first_name,
        last_name: &last_name,
    };

    println!("{:?}", name_view);
    println!("Full name: {}", name_view.full_name());
}
