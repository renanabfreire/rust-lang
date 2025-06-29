use std::io;

#[derive(Debug)]
struct Temperature {
    value: f64,
    schale: char,
}

impl Temperature {
    fn to_celsius(&mut self) {
        if self.schale == 'F' {
            self.schale = 'C';
            self.value = (self.value - 32.0) * 5.0 / 9.0;
        }
    }

    fn to_fahrenheit(&mut self) {
        if self.schale == 'C' {
            self.schale = 'F';
            self.value = self.value * 9.0 / 5.0 + 32.0;
        }
    }

    fn new(value: f64, schale: char) -> Self {
        Self { value, schale }
    }
}

fn main() {
    let mut buff = String::new();

    println!("_________________________________");
    println!("===== Temperature Conversor =====");
    println!("_________________________________");

    println!("Type the temperature you want to convert");

    io::stdin()
        .read_line(&mut buff)
        .expect("Error to read value");

    println!("---------------------------------");

    let buff = buff.trim();

    let sch = buff.chars().last();
    let v: f64 = buff[..(buff.len() - 1)]
        .parse()
        .expect("Not a numeric value");

    if let Some(sch) = sch {
        let mut temp = Temperature::new(v, sch);

        if sch == 'F' {
            temp.to_celsius();
        } else if sch == 'C' {
            temp.to_fahrenheit();
        } else {
            println!("Invalid type");
        }

        println!("This is equal to {}°{}", temp.value, temp.schale);
    } else {
        println!("Invalid type");
    }
}
