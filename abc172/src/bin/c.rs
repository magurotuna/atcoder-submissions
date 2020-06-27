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
fn solve() {
    let (n, m, k) = get!((usize, usize, usize));
    let As = get!([usize]);
    let Bs = get!([usize]);

    let mut a_cum = vec![0];
    for i in 0..n {
        let last = *a_cum.last().unwrap();
        a_cum.push(last + As[i]);
    }
    let mut b_cum = vec![0];
    for i in 0..m {
        let last = *b_cum.last().unwrap();
        b_cum.push(last + Bs[i]);
    }

    let mut ok = 0;
    let mut ng = 500_000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if readable(&a_cum, &b_cum, mid, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    echo!(ok);
}

fn readable(a_cum: &[usize], b_cum: &[usize], x: usize, k: usize) -> bool {
    let a_len = a_cum.len() - 1;
    let b_len = b_cum.len() - 1;
    for from_a in 0..=x {
        let from_b = x - from_a;
        if a_len < from_a || b_len < from_b {
            continue;
        }
        let a_time = a_cum[from_a];
        let b_time = b_cum[from_b];
        if a_time + b_time <= k {
            return true;
        }
    }

    false
}

fn solve2() {
    let (n, m, k) = get!((usize, usize, usize));
    let mut As = get!([usize]);
    let mut Bs = get!([usize]);
    As.reverse();
    Bs.reverse();
    let mut ans = 0;
    let mut minutes = 0;
    loop {
        debug!(As, Bs, ans, minutes);
        if minutes > k {
            break;
        }

        let a_num = As.len();
        let b_num = Bs.len();

        let a_top = As.last();
        let b_top = Bs.last();

        match (a_top, b_top) {
            (Some(&a), Some(&b)) if a < b => {
                minutes += a;
                As.pop();
                ans += 1;
            }
            (Some(&a), Some(&b)) if a > b => {
                minutes += b;
                Bs.pop();
                ans += 1;
            }
            (Some(&a), Some(&b)) => {
                // a == b
                minutes += a;
                ans += 1;
                let mut i = 2;
                let mut a_cur = As.get(a_num.checked_sub(i).unwrap_or(1_000_000_000));
                let mut b_cur = Bs.get(b_num.checked_sub(i).unwrap_or(1_000_000_000));
                let mut updated = false;
                while let (Some(&ac), Some(&bc)) = (a_cur, b_cur) {
                    if minutes > k {
                        echo!(ans - 1);
                        return;
                    }

                    if ac == bc {
                        minutes += ac;
                        ans += 1;
                        i += 1;
                        a_cur = As.get(a_num.checked_sub(i).unwrap_or(1_000_000_000));
                        b_cur = Bs.get(b_num.checked_sub(i).unwrap_or(1_000_000_000));
                        continue;
                    }
                    if ac < bc {
                        for _ in 0..(i - 1) {
                            As.pop();
                        }
                    } else {
                        for _ in 0..(i - 1) {
                            Bs.pop();
                        }
                    }
                    updated = true;
                    break;
                }
                if !updated {
                    if As.len() <= Bs.len() {
                        for _ in 0..As.len() {
                            Bs.pop();
                        }
                    } else {
                        for _ in 0..Bs.len() {
                            As.pop();
                        }
                    }
                }
                //ans += 1;
            }
            (Some(&a), None) => {
                minutes += a;
                As.pop();
                ans += 1;
            }
            (None, Some(&b)) => {
                minutes += b;
                Bs.pop();
                ans += 1;
            }
            (None, None) => break,
        }
    }

    if minutes > k {
        echo!(ans - 1);
    } else {
        echo!(ans);
    }
}
