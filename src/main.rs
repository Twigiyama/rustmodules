use std::io;
use rand::prelude::*;

fn main() {
    let mut buffer = String::new();
    println!("Enter a new line of text:");
    io::stdin().read_line(&mut buffer);
    println!("The text you entered is {buffer}");

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}
