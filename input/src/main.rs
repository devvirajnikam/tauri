use std::io;

fn simple_read_input()-> (){
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("can't read line");
    println!("Worked {}", command);
}

fn main(){
    simple_read_input();
}