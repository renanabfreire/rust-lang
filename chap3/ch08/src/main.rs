// Including io option of standard library
use std::io;

fn main() {
    println!("Set a number:"); // asking a value to the user

    // Defining number a mutable variable
    let mut number = String::new();

    // Reading the user's value to the variable number
    io::stdin()
        .read_line(&mut number)
        .expect("Not possible to read");

    // Converting it into a f64 type
    let mut number: f64 = number.trim().parse().expect("Invalid numeric value");

    // Having the square of the value
    number = square(number);

    // Printing the result
    println!("The square is equal to {number}");
}

// This function will get the f64 value
// and return it's square (value * value)
fn square(num: f64) -> f64 {
    // returning square
    num * num
}
