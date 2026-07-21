mod a_error;

fn main() {
    let result = a_error::with_strings::first_line_last_char().expect("Failed to get last character");
    println!("{:?}", result);
}
