fn main() {
    print_slice(&[6, 20, 2025]);
}

fn print_slice(slice: &[i32]) {
    for a in slice {
        println!("{a}");
    }
}
