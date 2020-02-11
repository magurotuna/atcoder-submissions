use libprocon::*;
use std::collections::BTreeMap;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        bc: [(usize, usize); M],
    }
    let mut counts = BTreeMap::new();
    for a in A.into_iter() {
        let entry = counts.entry(a).or_insert(0);
        *entry += 1;
    }

    for (b, c) in bc.into_iter() {
        let mut rest_b = b;
        let mut remove_key = Vec::new();
        let mut replace_key_value = None;
        for (&key, &count) in counts.iter() {
            if key >= c {
                break;
            }
            if count <= rest_b {
                // 全部cに置き換えることが可能
                rest_b = rest_b - count;
                remove_key.push(key);
            } else {
                // rest_b の分だけcに置き換えることが可能
                replace_key_value = Some((key, rest_b));
                rest_b = 0;
            }

            if rest_b == 0 {
                break;
            }
        }
        for key in remove_key {
            counts.remove(&key);
        }
        if let Some((k, v)) = replace_key_value {
            counts.entry(k).and_modify(|x| *x -= v);
        }
        *counts.entry(c).or_insert(0) += b - rest_b;
    }
    println!("{}", counts.into_iter().map(|(k, v)| k * v).sum::<usize>());
}
