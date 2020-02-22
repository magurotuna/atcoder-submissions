use libprocon::*;
use std::cmp::min;

fn main() {
    input! {
        N: usize,
        AB: [(i64, i64); N],
    }
    let mut ent = 0;
    let mut min_stoa = 1 << 60;
    for i in 0..N {
        let tmp_ent = AB[i].0;
        let total = AB.iter().map(|&(a, _)| (tmp_ent - a).abs()).sum::<i64>();
        if total < min_stoa {
            ent = tmp_ent;
            min_stoa = total;
        }
    }

    let mut exit = 0;
    let mut min_btoe = 1 << 60;
    for i in 0..N {
        let tmp_exit = AB[i].1;
        let total = AB.iter().map(|&(_, b)| (tmp_exit - b).abs()).sum::<i64>();
        if total < min_btoe {
            exit = tmp_exit;
            min_btoe = total;
        }
    }

    println!(
        "{}",
        min_stoa + min_btoe + AB.iter().map(|&(a, b)| (b - a).abs()).sum::<i64>()
    );
}
