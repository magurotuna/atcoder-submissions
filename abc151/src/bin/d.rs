use libprocon::*;
use std::collections::VecDeque;

fn main() {
    input! {
        H: usize,
        W: usize,
        maze: [chars; H],
    }

    let mut max_move_count = 0;

    for y in 0..H {
        for x in 0..W {
            if maze[y][x] == '#' {
                continue;
            }

            for v in 0..H {
                for u in 0..W {
                    if maze[v][u] == '#' {
                        continue;
                    }

                    let mut dist = vec![vec![-1; W]; H];
                    dist[y][x] = 0;

                    let mut q = VecDeque::new();
                    q.push_back((y, x));
                    while let Some((j, i)) = q.pop_front() {
                        if i == u && j == v {
                            // ゴールに到着
                            if dist[v][u] > max_move_count {}
                            max_move_count = std::cmp::max(max_move_count, dist[v][u]);
                            break;
                        }
                        let d = dist[j][i];
                        if i != 0 {
                            // 左に移動できる
                            if dist[j][i - 1] == -1 && maze[j][i - 1] == '.' {
                                q.push_back((j, i - 1));
                                dist[j][i - 1] = d + 1;
                            }
                        }
                        if i != W - 1 {
                            // 右に移動できる
                            if dist[j][i + 1] == -1 && maze[j][i + 1] == '.' {
                                q.push_back((j, i + 1));
                                dist[j][i + 1] = d + 1;
                            }
                        }

                        if j != 0 {
                            // 上に移動できる
                            if dist[j - 1][i] == -1 && maze[j - 1][i] == '.' {
                                q.push_back((j - 1, i));
                                dist[j - 1][i] = d + 1;
                            }
                        }
                        if j != H - 1 {
                            // 下に移動できる
                            if dist[j + 1][i] == -1 && maze[j + 1][i] == '.' {
                                q.push_back((j + 1, i));
                                dist[j + 1][i] = d + 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", max_move_count);
}
