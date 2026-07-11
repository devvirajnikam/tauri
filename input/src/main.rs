use std::io;

fn _simple_read_input()-> (){
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("can't read line");
    println!("Worked {}", command);
}

fn match_result()->(){
    let mut command = String::new();
    let result = io::stdin().read_line(&mut command);

    match &result {
        Ok(data) => {
            println!("Success, {:#?}", data);
            println!("The Input Was, {}", command);
            println!("The Result was. {:#?}", result);
        }
        Err(e)=>{
            println!("Failed, {:#?}", e)
        }
    }
}

fn main(){
    match_result();
}