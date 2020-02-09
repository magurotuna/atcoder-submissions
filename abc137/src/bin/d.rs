use libprocon::*;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

// AC 109ms
// かなり複雑に考えすぎた。BinaryHeapでもっとシンプルに解けた。
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); N],
    }
    // とりあえずソートしておく
    // Aについての降順、Aが等しい場合はBについての降順、とする
    let mut AB = AB;
    AB.sort_by(|x, y| match y.0.cmp(&x.0) {
        Ordering::Equal => y.1.cmp(&x.1),
        other => other,
    });
    // dbg!(&AB);

    // 方針
    // 最終日の1日前から順にさかのぼりながら考える
    // 最終日の1日前では、1日後に報酬がもらえるバイトのうち、一番報酬が高いものをやればOK
    // 最終日の2日前では、1日後or2日後に報酬がもらえるバイトのうち、一番報酬が高いものをやればOK. ただし、最終日1日前にやったバイトはやることができないことに注意
    // 以下同じように、貪欲にバイトを選択していけばよい。
    //
    // keyが「何日後に報酬がもらえるか」、valueがバイトの報酬のVec(降順）、となるようなHashMapを作る
    // また、セグメント木を作り、1日後、2日後、...、i日後に報酬がもらえるバイトのうち、一番報酬が高いものはどれか、というクエリをlogオーダーで解決できるようにしておく
    // これで O(nlogn) で解答できる はず。。。
    let mut hm = HashMap::with_capacity(N);
    let mut st = SegmentTree::new((0, 0), 100_000 + 1, |x, y| {
        if x.0 >= y.0 {
            (x.0, x.1)
        } else {
            (y.0, y.1)
        }
    });
    for (day, compensation) in AB.into_iter() {
        if !hm.contains_key(&day) {
            st.update(day, (compensation, day));
        }
        let entry = hm.entry(day).or_insert(VecDeque::new());
        entry.push_back(compensation);
    }

    // dbg!((
    //     st.query(1, 2),
    //     st.query(1, 3),
    //     st.query(1, 4),
    //     st.query(1, 5)
    // ));

    let mut ans = 0;
    for i in 1..=M {
        let res = st.query(1, i + 1);
        ans += res.0;
        if let Some(hm_el) = hm.get_mut(&res.1) {
            hm_el.pop_front();
            let next = hm.get(&res.1).unwrap().get(0).unwrap_or(&0);
            st.update(res.1, (*next, res.1));
        }
    }
    println!("{}", ans);
}
