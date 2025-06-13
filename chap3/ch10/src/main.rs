use std::io;

fn main() {
    println!("Set a numeric value");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error reading value");

    let number: f64 = number.trim().parse().expect("It wasn't a numeric value");

    if number < 10.0 {
        println!("{number} is small");
    } else if number < 100.0 {
        println!("{number} is medium");
    } else {
        println!("{number} is large");
    }
}
