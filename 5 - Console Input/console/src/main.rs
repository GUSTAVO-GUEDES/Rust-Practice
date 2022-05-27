use std::io;

fn main() {
    //Create a mutable string
    let mut input = String::new();

    print!("Write Something:\n");
    //"&mut input" -> Creates a mutable pointer reference the input variable and pass to the function 
    io::stdin().read_line(&mut input).expect("Failed to read line");

    print!("\nYour Input: {}\n", input);
}
