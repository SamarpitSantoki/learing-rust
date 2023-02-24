fn main() {
    let mut value = 1;

    loop {
        if value == 5 {
            break;
        }
        println!("{:?}",value);
        value = value + 1;

    }
}