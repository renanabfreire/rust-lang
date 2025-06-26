fn main() {
    let mut s = String::from("Hello");

    complete_sentece(&mut s);

    println!("{s}");
}

fn complete_sentece(st: &mut String) {
    st.push_str(", Rustaceans!");
}
