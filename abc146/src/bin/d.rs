use libprocon::*;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        N: usize,
        ab: [(usize1, usize1); N-1],
    }
    let orig_ab = ab.clone();

    let mut ab = ab
        .into_iter()
        .map(|(x, y)| if x > y { (y, x) } else { (x, y) })
        .collect::<Vec<_>>();
    ab.sort_by_key(|x| x.0);

    let mut nodes = vec![vec![]; N];
    for &(a, b) in ab.iter() {
        nodes[a].push(b);
    }

    let mut neighbor = vec![HashSet::new(); N];
    for &(a, b) in ab.iter() {
        neighbor[a].insert(b);
        neighbor[b].insert(a);
    }
    let K = neighbor.iter().map(|h| h.len()).max().unwrap();
    println!("{}", K);

    let mut colors = HashMap::new();

    dfs(&nodes, &mut colors, 9999999, 0);

    for pair in orig_ab.iter() {
        println!("{}", colors.get(pair).unwrap());
    }
}

fn dfs(
    nodes: &[Vec<usize>],
    colors: &mut HashMap<(usize, usize), usize>,
    prev_color: usize,
    cur_node_number: usize,
) {
    let mut next_color = 1;

    for &next_node_number in nodes[cur_node_number].iter() {
        if prev_color == next_color {
            next_color += 1;
        }
        colors.insert((cur_node_number, next_node_number), next_color);
        dfs(nodes, colors, next_color, next_node_number);
        next_color += 1;
    }
}
