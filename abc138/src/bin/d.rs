use libprocon::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Node {
    children: Vec<usize>, // 子供の頂点番号を格納
    counter: usize,       // この頂点を根とする部分木に含まれるカウンターの値にどれだけ加算させるか
}

fn main() {
    input! {
        N: usize,
        Q: usize,
        ab: [(usize1, usize1); N-1],
        px: [(usize1, usize); Q],
    }
    let mut nodes = vec![
        Node {
            children: vec![],
            counter: 0
        };
        N
    ];
    for (a, b) in ab.into_iter() {
        nodes[a].children.push(b);
        // bがaの子供である保証はないため、逆向きの辺も張っておく
        nodes[b].children.push(a);
    }

    for (p, x) in px.into_iter() {
        nodes[p].counter += x;
    }

    let mut values = vec![0; N];
    let mut visited = HashSet::with_capacity(N);
    dfs(&nodes, 0, 0, &mut values, &mut visited);

    println!(
        "{}",
        values
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn dfs(
    nodes: &[Node],
    cur_node_number: usize,
    cur_value: usize,
    values: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
) {
    let cur_node = &nodes[cur_node_number];
    visited.insert(cur_node_number);

    let updated_cur_value = cur_value + cur_node.counter;
    values[cur_node_number] = updated_cur_value;

    for &child_node_number in cur_node.children.iter() {
        if visited.contains(&child_node_number) {
            continue;
        }
        dfs(nodes, child_node_number, updated_cur_value, values, visited);
    }
}
