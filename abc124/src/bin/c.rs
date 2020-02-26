use libprocon::*;

fn main() {
    input! {
        s: chars,
    }

    let mut num_b = 0;
    let mut num_w = 0;

    // 先頭を0にするときの塗替え枚数
    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == '1' {
                num_b += 1;
            }
        } else {
            if s[i] == '0' {
                num_b += 1;
            }
        }
    }
    // 先頭を1にするときの塗替え枚数
    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == '0' {
                num_w += 1;
            }
        } else {
            if s[i] == '1' {
                num_w += 1;
            }
        }
    }

    println!("{}", std::cmp::min(num_b, num_w));
}
