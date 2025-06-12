use std::io;

fn main() {
    println!("What's your name?");

    let mut usr = String::new();

    io::stdin()
        .read_line(&mut usr)
        .expect("Error reading username");

    let usr = usr.trim();

    print_greeting(usr);
}

fn print_greeting(name: &str) {
    println!("Hello, {name}!");
}
