use std::io;

fn main() {
    println!("Write something!");

    let mut row = String::new();
    let mut num = 0;
    // let a: Vec<String> = vec![];
    
    io::stdin().read_line(&mut row)
        .expect("Failed to read line");

    num += row.split(" ").count();

    println!("The words are: {}", num);

    // loop {
        
    // }
}
