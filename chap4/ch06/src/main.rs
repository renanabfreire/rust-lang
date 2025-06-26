fn main() {
    let text = "Hello IG_IJ!";

    let first = first_word(text);

    println!("The first word of:");
    println!("{text}");
    println!("is {first}");
}

fn first_word(complete: &str) -> &str {
    let byte = complete.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &complete[..i];
        }
    }

    &complete[..]
}
