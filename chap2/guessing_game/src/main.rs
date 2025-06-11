use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the numeric value");

    let secret_number = rand::rng().random_range(1..=100);

    loop{
        println!("Your guess:");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error, unvailable value");

        let guess : u32 = match guess.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Numeric value, please");
                                continue;
                            }
                        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("Winner!");
                break;
            }
        }
    }
}
