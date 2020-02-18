use libprocon::*;

fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    use std::collections::HashMap;
    let mut hm = HashMap::new();
    for s in S {
        *hm.entry(s).or_insert(0) += 1;
    }

    let mut ans = Vec::new();
    let mut max_cnt = *hm.values().max().unwrap();
    for (s, v) in hm {
        if v == max_cnt {
            ans.push(s);
        }
    }
    ans.sort();
    println!("{}", ans.join("\n"));
}
