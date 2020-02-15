use libprocon::*;

fn main() {
    input! {
        s: chars,
    }
    use std::collections::HashMap;
    let mut hm = HashMap::new();
    for c in s {
        *hm.entry(c).or_insert(0) += 1;
    }
    println!(
        "{}",
        if hm.values().all(|&x| x == 2) {
            "Yes"
        } else {
            "No"
        }
    );
}
