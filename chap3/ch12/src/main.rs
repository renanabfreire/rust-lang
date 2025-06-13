fn main() {
    let mut arr: [u8; 10] = [0; 10];
    let mut counter = 2;

    arr[1] = 1;

    while counter < 10 {
        arr[counter] = arr[counter - 1] + arr[counter - 2];

        counter += 1;
    }

    for a in arr {
            println!("{a}");
    }
}
