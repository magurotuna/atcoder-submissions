use libprocon::*;

fn main() {
    input! {
        N: usize,
        ab: [(usize, usize); N],
    }
    let mut ab = ab;
    ab.sort_by_key(|x| x.1);
    dbg!(&ab);

    let mut cur_sum = 0;
    for i in 0..(N - 1) {
        cur_sum += ab[i].0;
        if ab[i].1 != ab[i + 1].1 {
            if cur_sum > ab[i].1 {
                println!("No");
                return;
            }
        }
    }

    cur_sum += ab[N - 1].0;
    if cur_sum > ab[N - 1].1 {
        println!("No");
        return;
    }
    println!("Yes");
}
