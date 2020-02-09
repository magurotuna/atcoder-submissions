use libprocon::*;

fn main() {
    input! {
        S: String,
    }
    let c = S.chars().map(|_| 'x').collect::<String>();
    println!("{}", c);
}
