use libprocon::*;
use std::cmp::min;

fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    let MOD = 1_000_000_007;
    // 二項係数に対応している
    // (X, Y) がパスカルの三角形上のどこに対応するのか対応するのか（= nCk で表すとn, kがいくつに対応するのか）がわかればおｋ

    // まず、そもそも(X, Y)に行けるかを判定する
    let reachable = ((X + Y) % 3 == 0) && (min(X, Y) >= (X + Y) / 3);
    if !reachable {
        println!("0");
        return;
    }

    // nCk の (n, k) を求める
    // nは (x + y) / 3
    let n = (X + Y) / 3;
    // kは (1, 2) 方向への移動を選んだ回数
    // 方程式をいい感じに解くと以下のように計算できる
    let k = (2 * Y - X) / 3;

    let comb = Comb::new(1_000_010, MOD);
    println!("{}", comb.calc(n, k));
}
