use libprocon::*;

fn main() {
    input! {
        S: String,
    }
    let d = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let i = d.iter().rev().position(|&x| x == S).unwrap();
    println!("{}", i + 1);
}
