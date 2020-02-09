use libprocon::*;

fn main() {
    input! {
        N: usize,
        K: usize,
        p: [usize; N],
    }

    let p = p.into_iter().map(|x| exp(x)).collect::<Vec<f64>>();

    let mut range_sum = 0.0;
    let mut max_sum = 0.0;
    for i in 0..(N - K + 1) {
        range_sum = if i == 0 {
            p.iter().take(K).sum()
        } else {
            range_sum - p[i - 1] + p[i + K - 1]
        };
        dbg!(range_sum);
        if range_sum > max_sum {
            max_sum = range_sum;
        }
    }
    println!("{}", max_sum);
}

fn exp(p: usize) -> f64 {
    let s = (p * (p + 1)) / 2;
    s as f64 / p as f64
}
