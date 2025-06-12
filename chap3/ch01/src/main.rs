fn main() {
    let mut counter: u8 = 0;

    loop {
        counter += 1;

        println!("{counter}");

        if counter >= 10 {
            break;
        }
    }
}
