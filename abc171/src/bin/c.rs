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
    let N = get!(usize);
    let mut v = vec![1];
    let mut cumsum = 1;
    let mut cur = 1;
    loop {
        if cur > N {
            break;
        }
        v.push(cumsum + cur * 26);
        cur *= 26;
        cumsum += cur;
    }
    debug!(v);
    let mut slide = None;
    let mut ans_len = None;
    for i in 0..(v.len() - 1) {
        let v1 = v[i];
        let v2 = v[i + 1];
        if v1 <= N && N < v2 {
            slide = Some(v1);
            ans_len = Some(i + 1);
            break;
        }
    }
    let slide = slide.unwrap();
    let ans_len = ans_len.unwrap();
    debug!(slide, ans_len);
    let mut num = N - slide;
    let mut ans = String::new();

    for _ in 0..ans_len {
        let rem = num % 26;
        debug!(num, rem);
        num /= 26;
        ans.push(i2a(rem));
    }

    echo!(ans.chars().rev().collect::<String>());

    //loop {
    //let div = num / 26;
    //let rem = num % 26;
    //ans.push(i2a(rem));
    //if div == 0 {
    //break;
    //} else {
    //num /= 26;
    //}
    //}
    //echo!(ans);
}

fn i2a(i: usize) -> char {
    ('a' as u8 + i as u8) as char
}
