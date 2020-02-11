use libprocon::*;

fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }
    let mut A = A;
    A.sort();

    let mut result = Vec::with_capacity(N);
    let mut iter = A.iter();
    loop {
        let first = iter.next();
        let second = iter.next();

        if first.is_none() {
            break;
        }

        if second.is_none() {
            result.push(*first.unwrap());
            break;
        }

        let &f = first.unwrap();
        let &s = second.unwrap();
        if f < 0 && s < 0 {
            result.push(-f);
            result.push(-s);
            continue;
        } else if f < 0 && s >= 0 {
            // fが負でsが正の場合
            // 絶対値を比較して、符号逆転させたほうが得になるかどうか判定する
            if f.abs() > s {
                result.push(-f);
                result.push(-s);
                continue;
            } else {
                result.push(f);
                result.push(s);
                continue;
            }
        } else {
            // f, sともに正
            result.push(f);
            result.push(s);
            continue;
        }
    }
    println!("{}", result.into_iter().sum::<i64>());
}
