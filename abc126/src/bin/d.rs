use libprocon::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    dist: usize,
}

fn main() {
    input! {
        N: usize,
        uvw: [(usize1, usize1, usize); N-1],
    }
    let mut nodes = vec![vec![]; N];
    for (u, v, w) in uvw.into_iter() {
        nodes[u].push(Edge { to: v, dist: w });
        nodes[v].push(Edge { to: u, dist: w });
    }
    // 深さ優先で色を決めていけばよさそう
    let mut colors = vec![0; N];
    let mut visited = HashSet::with_capacity(N);
    dfs(0, 0, &nodes, &mut colors, &mut visited);

    println!(
        "{}",
        colors
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn dfs(
    prev_dist: usize,
    cur_node_number: usize,
    nodes: &[Vec<Edge>],
    colors: &mut Vec<i32>,
    visited: &mut HashSet<usize>,
) {
    visited.insert(cur_node_number);
    if prev_dist % 2 == 0 {
        // 距離が偶数だったら色1にする
        colors[cur_node_number] = 1;
    }
    for e in nodes[cur_node_number].iter() {
        if visited.contains(&e.to) {
            continue;
        }
        dfs(prev_dist + e.dist, e.to, nodes, colors, visited);
    }
}
