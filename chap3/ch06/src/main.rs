use std::io;

fn main() {
    let mut temper = String::new();

    println!("What temper you want to convert?");

    io::stdin()
        .read_line(&mut temper)
        .expect("Error to read value");

    let mut temper: f64 = temper.trim().parse().expect("set a float");

    temper = celsius_to_fahrenheit(temper);

    println!("This in Fahrenheit is equal to {temper}");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}
