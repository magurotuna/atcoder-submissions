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
macro_rules !get {(@inner [$src :expr ] chars ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .chars () .collect ::<Vec <char >>() } } ;(@inner [$src :expr ] usize1 ) =>{{get !(@inner [$src ] usize ) -1 } } ;(@inner [$src :expr ] [usize1 ] ) =>{{get !(@inner [$src ] [usize ] ) .into_iter () .map (|v |v -1 ) .collect ::<Vec <usize >>() } } ;(@inner [$src :expr ] [[usize1 ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [usize1 ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [usize1 ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[chars ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [chars ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] chars ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [($($tt :tt ) ,*) ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] ($($tt ) ,*) ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] ($($tt :tt ) ,*) ) =>{{let mut buf :String =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;let mut iter =buf .split_whitespace () ;($(get !(@inner_elem_parse [$tt ] iter .next () .unwrap () ) ,) *) } } ;(@inner [$src :expr ] [$t :ty ] ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .map (|t |t .parse ::<$t >() .unwrap () ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [[$t :ty ] ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] [$t :ty ;$n :expr ] ) =>{{(0 ..$n ) .map (|_ |get !(@inner [$src ] [$t ] ) ) .flatten () .collect ::<Vec <_ >>() } } ;(@inner [$src :expr ] $t :ty ) =>{{let mut buf =String ::new () ;$src .read_line (&mut buf ) .unwrap () ;buf .trim () .split_whitespace () .next () .unwrap () .parse ::<$t >() .unwrap () } } ;(@inner_elem_parse [usize1 ] $elem :expr ) =>{{get !(@inner_elem_parse [usize ] $elem ) -1 } } ;(@inner_elem_parse [$t :ty ] $elem :expr ) =>{{$elem .parse ::<$t >() .unwrap () } } ;($tt :tt ) =>{{use std ::io ::BufRead ;let get_stdin =std ::io ::stdin () ;let mut locked_stdin =get_stdin .lock () ;get !(@inner [&mut locked_stdin ] $tt ) } } ;}
macro_rules !debug {($($a :expr ) ,*) =>{eprintln !(concat !($(stringify !($a ) ," = {:?}, " ) ,*) ,$($a ) ,*) ;} }
macro_rules !echo {($($a :expr ) ,*) =>{let mut s =Vec ::new () ;$(s .push (format !("{}" ,$a ) ) ;) *println !("{}" ,s .join (" " ) ) ;} }
#[macro_export]
macro_rules !chmin {($base :ident ,$($cmps :expr ) ,+$(,) *) =>{$base =min !($base ,$($cmps ) ,+) ;} ;}
#[macro_export]
macro_rules !chmax {($base :ident ,$($cmps :expr ) ,+$(,) *) =>{$base =max !($base ,$($cmps ) ,+) ;} ;}
#[macro_export]
macro_rules !min {($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::min ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::min ($a ,min !($($rest ) ,+) ) } } ;}
#[macro_export]
macro_rules !max {($a :expr ,$b :expr $(,) *) =>{{std ::cmp ::max ($a ,$b ) } } ;($a :expr ,$($rest :expr ) ,+$(,) *) =>{{std ::cmp ::max ($a ,max !($($rest ) ,+) ) } } ;}
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
    let n = get!(usize);
    let s = get!(chars);
    let rh = RollingHash::new(&s);

    let mut ok = 0;
    let mut ng = s.len() / 2 + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if can(mid, &rh, n) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    echo!(ok);
}

// are there any substrings of `len` length that satisfy the given condition?
fn can(len: usize, rh: &RollingHash, n: usize) -> bool {
    let mut set = HashSet::new();
    for i in len..(n - len + 1) {
        set.insert(rh.get(i - len, i));
        if set.contains(&rh.get(i, i + len)) {
            return true;
        }
    }

    false
}

#[derive(Debug)]
pub struct RHMod(u64);
#[derive(Debug)]
pub struct RHBase(u64);
#[derive(Debug)]
struct RHInner {
    hash: u64,
    power: u64,
}
impl RHInner {
    fn new(hash: u64, power: u64) -> RHInner {
        RHInner {
            hash: hash,
            power: power,
        }
    }
}
#[derive(Debug)]
pub struct RollingHash {
    hash_pow_list: Vec<(RHMod, Vec<RHInner>)>,
}
impl RollingHash {
    pub fn new(target: &[char]) -> RollingHash {
        RollingHash::with_base_mod(
            target,
            &[
                (RHBase(2315961251), RHMod(4294966367)),
                (RHBase(1692999586), RHMod(4294959359)),
                (RHBase(1009), RHMod(1_000_000_007)),
            ],
        )
    }
    fn with_base_mod(target: &[char], base_mod: &[(RHBase, RHMod)]) -> RollingHash {
        let hp_list = base_mod
            .iter()
            .map(|&(RHBase(base), RHMod(modulo))| {
                let mut hp = Vec::with_capacity(target.len() + 1);
                hp.push(RHInner::new(0, 1));
                for (i, &c) in target.iter().enumerate() {
                    let RHInner { hash, power } = hp[i];
                    let next_hash = (hash + c as u64) * base % modulo;
                    let next_power = power * base % modulo;
                    hp.push(RHInner::new(next_hash, next_power));
                }
                (RHMod(modulo), hp)
            })
            .collect();
        RollingHash {
            hash_pow_list: hp_list,
        }
    }
    pub fn get(&self, left: usize, right: usize) -> Vec<u64> {
        self.hash_pow_list
            .iter()
            .map(|&(RHMod(modulo), ref hp)| {
                (hp[right].hash + modulo - hp[left].hash * hp[right - left].power % modulo) % modulo
            })
            .collect()
    }
}
