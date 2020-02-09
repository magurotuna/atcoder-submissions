use libprocon::*;

fn main() {
    input! {
        S: String,
    }
    let chars = S.chars().collect::<Vec<_>>();
    let chars_len = chars.len();
    // ランレングス圧縮する
    // rl[i] := (iが偶数なら)連続しているRの数、(iが奇数なら)連続しているLの数
    let mut rl = Vec::with_capacity(chars.len());

    let mut cur_c = chars[0];
    let mut cur_len = 1;
    for c in chars.into_iter().skip(1) {
        if cur_c == c {
            cur_len += 1;
            continue;
        } else {
            cur_c = c;
            rl.push(cur_len);
            cur_len = 1;
        }
    }
    rl.push(cur_len);
    assert!(rl.len() % 2 == 0);

    dbg!(&rl);

    let mut ans_vec = Vec::with_capacity(chars_len);
    for i in (0..(rl.len() - 1)).step_by(2) {
        let r_num = rl[i];
        let l_num = rl[i + 1];

        for _ in 0..(r_num - 1) {
            ans_vec.push(0);
        }
        if (r_num + l_num) % 2 == 0 {
            ans_vec.push((r_num + l_num) / 2);
            ans_vec.push((r_num + l_num) / 2);
        } else {
            // 平衡状態になるとき(max(r, l) - 1 回の移動が終わったタイミング)でceil((r + l) / 2) or floor((r + l) / 2)人がいる状態になる（rとlのどっちが大きいかで変わる）
            // max(r, l) - 1 が偶数なら、10^100回の移動後はこの値そのままでOK. 奇数ならrとlが逆転する
            let ceil = (r_num + l_num + 1) / 2;
            let floor = (r_num + l_num) / 2;
            use std::cmp::max;
            if (max(r_num, l_num) - 1) % 2 == 0 {
                ans_vec.push(if r_num > l_num { ceil } else { floor });
                ans_vec.push(if r_num > l_num { floor } else { ceil });
            } else {
                ans_vec.push(if r_num > l_num { floor } else { ceil });
                ans_vec.push(if r_num > l_num { ceil } else { floor });
            }
        }
        for _ in 0..(l_num - 1) {
            ans_vec.push(0);
        }
    }

    println!(
        "{}",
        ans_vec
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
