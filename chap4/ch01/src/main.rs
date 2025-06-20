fn main() {
    // Here I create a variable that I'm going to use to test ownership throughout the file.
    let s = String::from("Ownership");

    // Here I shadow the above 's' and create a new 's' that takes ownership
    // back form the function 'take_ownership' after it has used it.
    let s = take_ownership(s);

    // This will that everything is working as expected.
    println!("{s} understood :)");
} // variables are dropped.

fn take_ownership(st: String) -> String {
    // Here I take ownership of st.

    // [..] <- function

    // After using it, I have to return the variable. Otherwise, the function would drop it.
    st
} // Here varables used locally are dropped, but 'st' was returned before

/*
wrong way:

fn take_ownership(st: String) {
    [...]
} here, I would drop the variable that I expected to use in the main function.
*/
