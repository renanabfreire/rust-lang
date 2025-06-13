fn main() {
    let mut counter = 5;

    loop {
        println!("{counter}");

        counter -= 1;
        if counter < 1 {
            break;
        }
    }

    println!("Liftoff!");
}
