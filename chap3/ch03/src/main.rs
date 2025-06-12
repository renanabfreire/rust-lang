use std::io;

fn main() {
    let mut read = String::new();

    println!("Type an integer:");

    io::stdin().read_line(&mut read).expect("Reading error");

    let read: i32 = read.trim().parse().expect("invalid input value");

    println!("The number times 10 is equal to {}", read * 10);
}
