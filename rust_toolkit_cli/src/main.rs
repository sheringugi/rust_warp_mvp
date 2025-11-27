use std::io; // import input/output library

fn main() {
    println!("Welcome to Rust Toolkit MVP!");
    println!("Enter your name:");

    let mut name = String::new(); // create a mutable string to store user input
    io::stdin().read_line(&mut name).expect("Failed to read line"); // read input

    let name = name.trim(); // Trim whitespace from the input

    println!("Hello, {}! This is your Rust MVP.", name); // print greeting
    println!("\nNow, see your greeting in the web app!");
    println!("Visit: http://127.0.0.1:3000/?name={}", name);
}
