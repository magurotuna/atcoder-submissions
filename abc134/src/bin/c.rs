use libprocon::*;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut st = SegmentTree::new(0, 200_000, max);
    for i in 0..n {
        st.update(i, a[i]);
    }

    for i in 0..n {
        println!("{}", max(st.query(0, i), st.query(i + 1, n)));
    }
}
