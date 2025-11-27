use std::io; // import input/output library

fn main() {
    println!("Welcome to Rust Toolkit MVP!");
    println!("Enter your name:");

    let mut name = String::new(); // create a mutable string to store user input
    io::stdin().read_line(&mut name).expect("Failed to read line"); // read input

    println!("Hello, {}! This is your Rust MVP.", name.trim()); // print greeting
}
