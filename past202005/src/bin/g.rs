//! https://github.com/hatoo/competitive-rust-snippets
//!
//! MIT License
//!
//! Copyright (c) 2018 hatoo
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
#![allow(
    unused_imports,
    unused_attributes,
    unused_macros,
    dead_code,
    non_snake_case
)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::FromIterator;
#[macro_export]
macro_rules ! get {(@ inner [$ src : expr ] chars ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . chars () . collect ::< Vec < char >> () } } ; (@ inner [$ src : expr ] usize1 ) => {{get ! (@ inner [$ src ] usize ) - 1 } } ; (@ inner [$ src : expr ] [usize1 ] ) => {{get ! (@ inner [$ src ] [usize ] ) . into_iter () . map (| v | v - 1 ) . collect ::< Vec < usize >> () } } ; (@ inner [$ src : expr ] [[usize1 ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [usize1 ] ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [usize1 ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [usize1 ] ) ) . flatten () . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [[chars ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] chars ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [chars ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] chars ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [($ ($ tt : tt ) ,* ) ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] ($ ($ tt ) ,* ) ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] ($ ($ tt : tt ) ,* ) ) => {{let mut buf : String = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; let mut iter = buf . split_whitespace () ; ($ (get ! (@ inner_elem_parse [$ tt ] iter . next () . unwrap () ) , ) * ) } } ; (@ inner [$ src : expr ] [$ t : ty ] ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . split_whitespace () . map (| t | t . parse ::<$ t > () . unwrap () ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [[$ t : ty ] ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [$ t ] ) ) . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] [$ t : ty ; $ n : expr ] ) => {{(0 ..$ n ) . map (| _ | get ! (@ inner [$ src ] [$ t ] ) ) . flatten () . collect ::< Vec < _ >> () } } ; (@ inner [$ src : expr ] $ t : ty ) => {{let mut buf = String :: new () ; $ src . read_line (& mut buf ) . unwrap () ; buf . trim () . split_whitespace () . next () . unwrap () . parse ::<$ t > () . unwrap () } } ; (@ inner_elem_parse [usize1 ] $ elem : expr ) => {{get ! (@ inner_elem_parse [usize ] $ elem ) - 1 } } ; (@ inner_elem_parse [$ t : ty ] $ elem : expr ) => {{$ elem . parse ::<$ t > () . unwrap () } } ; ($ tt : tt ) => {{use std :: io :: BufRead ; let get_stdin = std :: io :: stdin () ; let mut locked_stdin = get_stdin . lock () ; get ! (@ inner [& mut locked_stdin ] $ tt ) } } ; }
macro_rules ! debug {($ ($ a : expr ) ,* $ (, ) * ) => {# [cfg (debug_assertions ) ] eprintln ! (concat ! ($ ("| " , stringify ! ($ a ) , "={:?} " ) ,*, "|" ) , $ (&$ a ) ,* ) ; } ; }
macro_rules ! echo {($ ($ a : expr ) ,* ) => {let mut s = Vec :: new () ; $ (s . push (format ! ("{}" , $ a ) ) ; ) * println ! ("{}" , s . join (" " ) ) ; } }
#[macro_export]
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
#[macro_export]
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[macro_export]
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
const BIG_STACK_SIZE: bool = true;
fn main() {
    use std::thread;
    if BIG_STACK_SIZE {
        thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .name("solve".into())
            .spawn(solve)
            .unwrap()
            .join()
            .unwrap();
    } else {
        solve();
    }
}

fn solve3() {
    let (N, X, Y) = get!((usize, i64, i64));
    let mut memo = HashMap::new();
    memo.insert((0, 0), 0);
    let INF: usize = 1 << 60;
    for i in -210..210 {
        memo.insert((i, 210), INF);
        memo.insert((210, i), INF);
    }
    let xy = get!([(i64, i64); N]);
    for (x, y) in xy {
        memo.insert((x, y), INF);
    }
    echo!(calc(&mut memo, X, Y));
}

fn calc(memo: &mut HashMap<(i64, i64), usize>, x: i64, y: i64) -> i64 {
    debug!(x, y);
    if memo.contains_key(&(x, y)) {
        return memo[&(x, y)] as i64;
    }
    let mut min_cost: usize = 1 << 60;
    chmin!(min_cost, calc(memo, x, y + 1) as usize);
    chmin!(min_cost, calc(memo, x + 1, y) as usize);
    chmin!(min_cost, calc(memo, x + 1, y - 1) as usize);
    chmin!(min_cost, calc(memo, x, y - 1) as usize);
    chmin!(min_cost, calc(memo, x - 1, y - 1) as usize);
    chmin!(min_cost, calc(memo, x - 1, y) as usize);

    if min_cost == 1 << 60 {
        memo.insert((x, y), min_cost);
        -1
    } else {
        memo.insert((x, y), min_cost + 1);
        min_cost as i64 + 1
    }
}

fn solve2() {
    let (N, X, Y) = get!((usize, i64, i64));
    let X = (X + 200) as usize;
    let Y = (Y + 200) as usize;
    let mut maze = vec![vec![0; 410]; 410];
    let xy = get!([(i64, i64); N]);
    for (x, y) in xy {
        let x = x + 200;
        let y = y + 200;
        maze[x as usize][y as usize] = -1;
    }
    maze[200][200] = -2; // mark as start

    let mut queue = VecDeque::new();
    queue.push_back((200, 200, 0)); // x, y, cost

    while let Some((cur_x, cur_y, cost)) = queue.pop_front() {
        if cur_x == X && cur_y == Y {
            echo!(cost);
            return;
        }
        maze[cur_x][cur_y] = cost;
        for &(dx, dy) in [(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)].iter() {
            if dx < 0 && cur_x == 0 {
                continue;
            }
            if dy < 0 && cur_y == 0 {
                continue;
            }
            if dx + cur_x as i64 >= 410 {
                continue;
            }
            if dy + cur_y as i64 >= 410 {
                continue;
            }

            let nx = (cur_x as i64 + dx) as usize;
            let ny = (cur_y as i64 + dy) as usize;
            if maze[nx][ny] != 0 {
                continue;
            }
            queue.push_back((nx, ny, cost + 1));
        }
    }

    echo!(-1);
}

fn solve() {
    let (N, X, Y) = get!((usize, i64, i64));
    let xy: HashSet<_> = get!([(i64, i64); N]).into_iter().collect();
    let mut queue = VecDeque::new();
    queue.push_back(Position {
        x: 0,
        y: 0,
        shortest: 0,
    });
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while let Some(Position { x, y, shortest }) = queue.pop_front() {
        //debug!(x, y);
        if x == X && y == Y {
            echo!(shortest);
            return;
        }

        for &(dx, dy) in [(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)].iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx.abs() >= 205 || ny.abs() >= 205 {
                continue;
            }
            if visited.contains(&(nx, ny)) {
                continue;
            }
            if xy.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            queue.push_back(Position {
                x: nx,
                y: ny,
                shortest: shortest + 1,
            });
        }
    }
    echo!(-1);
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    shortest: usize,
}
