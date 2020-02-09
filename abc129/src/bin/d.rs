use libprocon::*;

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [chars; H],
    }

    // マスごとに上下方向、左右方向にいくつ照らすことができるかを保持するベクタ
    let mut counts = vec![vec![(0, 0); W]; H];

    let mut ans = 0;
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == '#' {
                continue;
            }

            let horizontal = if w == 0 || S[h][w - 1] == '#' {
                let mut diff_w = 1;
                while w + diff_w < W && S[h][w + diff_w] != '#' {
                    diff_w += 1;
                }
                diff_w
            } else {
                counts[h][w - 1].1
            };

            let vertical = if h == 0 || S[h - 1][w] == '#' {
                let mut diff_h = 1;
                while h + diff_h < H && S[h + diff_h][w] != '#' {
                    diff_h += 1;
                }
                diff_h
            } else {
                counts[h - 1][w].0
            };

            counts[h][w] = (vertical, horizontal);

            ans = std::cmp::max(ans, vertical + horizontal - 1);
        }
    }

    println!("{}", ans);
}
