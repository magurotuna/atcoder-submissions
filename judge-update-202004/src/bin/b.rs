use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        N: usize,
        mut xc: [(i32, char); N],
    }
    xc.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        otherwise => otherwise,
    });

    for i in 0..xc.len() {
        println!("{}", xc[i].0);
    }
}
