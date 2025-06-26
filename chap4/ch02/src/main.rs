fn main() {
    // Here I'm creating a string with 'Olá'
    let s = String::from("Olá");

    // Calling the function that modifies 's'
    let s = append_function(s);

    println!("{s}");
}

fn append_function(mut st: String) -> String {
    // Adding the rest of the phrase
    st.push_str(", World!");

    // Returning modified string
    st
}
