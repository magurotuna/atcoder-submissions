#![allow(
    unused_imports,
    unused_attributes,
    unused_macros,
    dead_code,
    non_snake_case
)]
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    sizes: Vec<i64>,
    rank: Vec<i64>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sizes: vec![1; n],
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> Option<usize> {
        if x >= self.parent.len() {
            None
        } else if self.parent[x] == x {
            Some(x)
        } else {
            let px = self.parent[x];
            let root = self.root(px).unwrap();
            self.parent[x] = root;
            Some(root)
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = match self.root(x) {
            None => return,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return,
            Some(val) => val,
        };
        if x_root == y_root {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x_root] = y_root;
            self.sizes[y_root] += self.sizes[x_root];
        } else {
            self.parent[y_root] = x_root;
            self.sizes[x_root] += self.sizes[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = match self.root(x) {
            None => return false,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return false,
            Some(val) => val,
        };

        x_root == y_root
    }

    pub fn size(&mut self, x: usize) -> Option<i64> {
        self.root(x).map(|r| self.sizes[r])
    }
}

// TODO: TLE!!! Must be reviewed.
fn main() {
    input! {
        H: usize,
        W: usize,
        ch: usize,
        cw: usize,
        dh: usize,
        dw: usize,
        s: [Chars; H],
    }

    let flat_number = |x: usize, y: usize| x + y * W;

    let mut regions = DisjointSet::new(H * W);
    for x in 0..W {
        for y in 0..H {
            if s[y][x] == '#' {
                continue;
            }

            let cur = flat_number(x, y);

            // up
            if y > 0 && s[y - 1][x] == '.' {
                regions.unite(cur, flat_number(x, y - 1));
            }
            // down
            if y < H - 1 && s[y + 1][x] == '.' {
                regions.unite(cur, flat_number(x, y + 1));
            }
            // right
            if x < W - 1 && s[y][x + 1] == '.' {
                regions.unite(cur, flat_number(x + 1, y));
            }
            // left
            if x > 0 && s[y][x - 1] == '.' {
                regions.unite(cur, flat_number(x - 1, y));
            }
        }
    }

    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for x in 0..W {
        for y in 0..H {
            if s[y][x] == '#' {
                continue;
            }

            let cur_region = regions.root(flat_number(x, y)).unwrap();

            // check for 5x5
            for i in (x as i32 - 2)..=(x as i32 + 2) {
                if i < 0 || i >= W as i32 {
                    continue;
                }
                let i = i as usize;
                for j in (y as i32 - 2)..=(y as i32 + 2) {
                    if j < 0 || j >= H as i32 {
                        continue;
                    }
                    let j = j as usize;
                    let state = s[j][i];
                    if state == '#' {
                        continue;
                    }

                    let target_region = regions.root(flat_number(i, j)).unwrap();

                    if cur_region == target_region {
                        continue;
                    }

                    graph
                        .entry(cur_region)
                        .or_insert_with(HashSet::new)
                        .insert(target_region);
                    graph
                        .entry(target_region)
                        .or_insert_with(HashSet::new)
                        .insert(cur_region);
                }
            }
        }
    }

    let start_num = flat_number(cw - 1, ch - 1);
    let goal_num = flat_number(dw - 1, dh - 1);
    let start_region = regions.root(start_num).unwrap();
    let goal_region = regions.root(goal_num).unwrap();

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((start_region, 0)); // (region_num, count of wrap)

    while let Some((cur_region, count)) = q.pop_front() {
        if cur_region == goal_region {
            println!("{}", count);
            return;
        }

        visited.insert(cur_region);

        if let Some(next_regions) = graph.get(&cur_region) {
            for &next in next_regions {
                if visited.contains(&next) {
                    continue;
                }
                q.push_back((next, count + 1));
            }
        } else {
            break;
        }
    }
    println!("-1");
}
