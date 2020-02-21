use libprocon::*;
use std::cmp::max;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut set = HashSet::new();
    for &(x, y) in xy.iter() {
        set.insert((x, y));
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (0..n).filter(|&x| x != i) {
            let xj = xy[j].0;
            let yj = xy[j].1;
            let xi = xy[i].0;
            let yi = xy[i].1;
            let d2 = (xj - xi).pow(2u32) + (yj - yi).pow(2u32);

            let xq = xj + yi - yj;
            let yq = yj + xj - xi;
            let xr = xi + yi - yj;
            let yr = yi + xj - xi;
            if set.contains(&(xq, yq)) && set.contains(&(xr, yr)) {
                ans = max(ans, d2);
            }
        }
    }

    println!("{}", ans);
}
