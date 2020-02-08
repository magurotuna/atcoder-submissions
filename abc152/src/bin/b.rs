use libprocon::*;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    use std::char::from_digit;
    let ac: char = from_digit(a, 10).unwrap();
    let bc: char = from_digit(b, 10).unwrap();

    let sa: String = (0..b).map(|_| ac).collect();
    let sb: String = (0..a).map(|_| bc).collect();

    println!("{}", if sa >= sb { &sb } else { &sa });
}
