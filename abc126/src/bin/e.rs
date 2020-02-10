use libprocon::*;

fn main() {
    input! {
        N: usize,
        M: usize,
        xyz: [(usize1, usize1, usize); M],
    }
    let mut ds = DisjointSet::new(N);
    for (x, y, _) in xyz.into_iter() {
        ds.unite(x, y);
    }
    use std::collections::HashSet;
    let mut h = HashSet::new();
    for i in 0..N {
        let root = ds.root(i).unwrap();
        h.insert(root);
    }
    println!("{}", h.len());
}

// 以下、自力で解いた結果 WA (14/20)
// 目の付け所（グループ数を数えればよい）は正しいが、以下のように数えると、例えば a-b をグループ1、c-d をグループ2、としたあと、b-c が同一グループでした！という事実が明らかになった場合、崩壊する
// 連結成分の個数を数えるには Union Find を使うのが手っ取り早い。
fn wa() {
    input! {
        N: usize,
        M: usize,
        xyz: [(usize1, usize1, usize); M],
    }
    let mut groups = vec![None; N];

    let mut n_groups = 0; // グループ数
    for (x, y, _) in xyz.into_iter() {
        if groups[x].is_none() && groups[y].is_none() {
            n_groups += 1;
            groups[x] = Some(n_groups);
            groups[y] = Some(n_groups);
        }

        if let Some(g) = groups[x] {
            groups[y] = Some(g);
        }
        if let Some(g) = groups[y] {
            groups[x] = Some(g);
        }
    }

    let none_count = groups.iter().filter(|g| g.is_none()).count();
    println!("{}", none_count + n_groups);
}
