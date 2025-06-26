fn main() {
    let mut s = String::from("Hey dude");

    {
        let r1 = &s;
        println!("{r1}");
    }

    let r2 = &mut s;
    println!("{r2}");
}
