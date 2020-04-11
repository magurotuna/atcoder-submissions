use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let tmp = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let N = tmp[0];
    let M = tmp[1];
    buf.clear();

    let LRD = {
        let mut ret = Vec::with_capacity(M);
        for i in 0..M {
            handle.read_line(&mut buf).unwrap();
            let tmp = buf
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            ret.push((tmp[0] - 1, tmp[1] - 1, tmp[2] as i64));
            buf.clear();
        }
        ret
    };

    let mut edges = Vec::new();
    for _ in 0..N {
        edges.push(vec![]);
    }
    for &(l, r, d) in &LRD {
        edges[l].push((r, d));
        edges[r].push((l, -d));
    }

    println!("{}", if solve2(N, &edges) { "Yes" } else { "No" });
}

fn solve(N: usize, edges: &Vec<Vec<(usize, i64)>>) -> bool {
    let mut visited = HashSet::new();
    for i in 0..N {
        if visited.contains(&i) {
            continue;
        }
        let mut rel_pos = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((i, 0));
        while let Some((n, pos)) = queue.pop_front() {
            if visited.contains(&n) {
                continue;
            }
            rel_pos.insert(n, pos);
            visited.insert(n);
            for &(to, d) in &edges[n] {
                if let Some(&r) = rel_pos.get(&to) {
                    if pos + d != r {
                        return false;
                    }
                }
                queue.push_back((to, pos + d));
            }
        }
    }
    true
}

fn solve2(N: usize, edges: &Vec<Vec<(usize, i64)>>) -> bool {
    let mut visited = HashSet::new();
    let mut rel_pos = vec![None; N];
    for i in 0..N {
        if visited.contains(&i) {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back((i, 0));
        while let Some((n, pos)) = queue.pop_front() {
            if visited.contains(&n) {
                continue;
            }
            rel_pos[n] = Some(pos);
            visited.insert(n);
            for &(to, d) in &edges[n] {
                if let Some(r) = rel_pos[to] {
                    if pos + d != r {
                        return false;
                    }
                }
                queue.push_back((to, pos + d));
            }
        }
    }
    true
}
