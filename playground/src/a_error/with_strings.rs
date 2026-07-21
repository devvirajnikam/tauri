pub fn get_char(){
    let s = String::from("hello");
    let c = s.chars().nth(2).expect("Access out of bound");
    println!("{:?}", c);
}


pub fn first_line_last_char() -> Option<char> {
    let data = "Some Random".lines().next()?.chars().last();
    println!("{:?}", data);
    data
}
