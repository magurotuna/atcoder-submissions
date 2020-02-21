use libprocon::*;

fn main() {
    input! {
        N: usize,
        S: chars,
    }
    let mut ans = 0;
    // 暗証番号は000から999の1000通りしかありえない
    for i in 0..1000 {
        let pin = format!("{:03}", i).chars().collect::<Vec<_>>();
        // pin を作れるか？
        let mut ok = true;
        let mut it = S.iter();
        'f: for p in pin {
            while let Some(&s) = it.next() {
                if p == s {
                    continue 'f;
                }
            }
            ok = false;
        }

        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
