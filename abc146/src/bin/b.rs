use libprocon::*;

fn main() {
    input! {
        N: u8,
        S: chars,
    }
    let cA = 'A' as u8;
    dbg!(cA);

    println!(
        "{}",
        S.into_iter()
            .map(|c| ((((c as u8 - cA) + N) % 26) + cA) as char)
            .collect::<String>()
    );
}
