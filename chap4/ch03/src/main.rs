fn main() {
    let s = String::from("Salve");

    let length = take_len(&s);

    println!("{s} still here and have {length} characters");
}

fn take_len(st: &String) -> usize {
    st.len()
}
