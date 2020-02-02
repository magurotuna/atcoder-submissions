use libprocon::*;

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Vertex {
    n_links: usize,
    inflows: Vec<usize>,
    outflows: Vec<usize>,
}

fn main() {
    let (N, M) = read!(usize, usize);
    let mut edges: Vec<Edge> = Vec::with_capacity(M);
    for _ in 0..M {
        let (from, to) = read!(usize, usize);
        edges.push(Edge {
            from: from - 1,
            to: to - 1,
        }); // 0-based
    }

    let mut vertexes: Vec<Vertex> = (0..N)
        .map(|_| Vertex {
            n_links: 0,
            inflows: vec![],
            outflows: vec![],
        })
        .collect();
    for edge in edges.iter() {
        vertexes[edge.to].n_links += 1;
        vertexes[edge.to].inflows.push(edge.from);
        vertexes[edge.from].outflows.push(edge.to);
    }

    let mut no_inflows = vertexes
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.n_links == 0 { Some(i) } else { None })
        .collect::<Vec<_>>();
    let mut sorted = vec![];

    while !no_inflows.is_empty() {
        let n = no_inflows.pop().unwrap();
        sorted.push(n);
        for &out in vertexes[n].outflows.clone().iter() {
            vertexes[out].n_links -= 1;
            if vertexes[out].n_links == 0 {
                no_inflows.push(out);
            }
        }
    }

    // dp[i] := 頂点iにいくための最長経路の長さ
    let mut dp = vec![-1_i32; N];
    let mut ans = 0;
    for &s in sorted.iter() {
        let len_longest = match vertexes[s].inflows.iter().map(|&i| dp[i]).max() {
            Some(-1) | None => 0,
            Some(x) => x + 1,
        };
        dp[s] = len_longest;
        ans = std::cmp::max(ans, len_longest);
    }

    println!("{}", ans);
}
