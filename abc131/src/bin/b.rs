use libprocon::*;

fn main() {
    input! {
        n: usize,
        l: i64,
    }
    let tastes = (1..=n).map(|x| l + (x as i64) - 1).collect::<Vec<_>>();

    let mut copied = tastes
        .iter()
        .copied()
        .enumerate()
        .map(|(x, y)| (x, y.abs()))
        .collect::<Vec<_>>();
    copied.sort_by_key(|x| x.1);

    let abs_min_index = copied[0].0;

    let mut ans = 0;
    for i in 0..n {
        if i == abs_min_index {
            continue;
        }
        ans += tastes[i];
    }
    println!("{}", ans);
}
