use libprocon::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
        M: usize,
        ps: [(usize, String); M],
    }

    let mut wa_count = 0;

    let mut ac = HashSet::new();
    let mut wa = HashMap::new();

    for a in ps {
        if a.1 == "AC" {
            if ac.contains(&a.0) {
                continue;
            }
            ac.insert(a.0);
        } else {
            if !ac.contains(&a.0) {
                let wa_cnt = wa.entry(a.0).or_insert(0);
                *wa_cnt += 1;
            }
        }
    }

    println!(
        "{} {}",
        ac.len(),
        wa.into_iter()
            .filter(|(ref key, _)| ac.contains(key))
            .map(|(_, val)| val)
            .sum::<usize>()
    );
}
