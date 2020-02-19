use libprocon::*;

fn main() {
    input! {
        S: chars,
    }
    let mut cur_max = 0;
    let mut tmp_cnt = 0;
    for c in S {
        if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
            tmp_cnt += 1;
        } else {
            cur_max = std::cmp::max(cur_max, tmp_cnt);
            tmp_cnt = 0;
        }
    }
    cur_max = std::cmp::max(cur_max, tmp_cnt);
    println!("{}", cur_max);
}
