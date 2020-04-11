use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
enum Query {
    Update(i64, i64),
    Calc,
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let Q = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut queries = Vec::with_capacity(Q);
    for _ in 0..Q {
        handle.read_line(&mut buf);
        {
            let mut it = buf
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap());
            if it.next().unwrap() == 1 {
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                queries.push(Query::Update(a, b));
            } else {
                queries.push(Query::Calc);
            }
        }
        buf.clear();
    }

    let mut b_sum = 0;
    let mut first_half_maxheap: BinaryHeap<i64> = BinaryHeap::new();
    let mut latter_half_minheap: BinaryHeap<i64> = BinaryHeap::new();
    let mut first_half_sum = 0;
    let mut latter_half_sum = 0;

    for q in queries {
        match q {
            Query::Calc => {
                let x = first_half_maxheap.peek().unwrap();
                //dbg!(
                //&x,
                //first_half_maxheap.len(),
                //first_half_sum,
                //latter_half_minheap.len(),
                //latter_half_sum
                //);
                let ans = (x * first_half_maxheap.len() as i64 - first_half_sum)
                    + (-x * latter_half_minheap.len() as i64 + latter_half_sum)
                    + b_sum;
                println!("{} {}", x, ans);
            }
            Query::Update(a, b) => {
                b_sum += b;
                if first_half_maxheap.is_empty() {
                    first_half_maxheap.push(a);
                    first_half_sum += a;
                } else if first_half_maxheap.len() > latter_half_minheap.len() {
                    let &t = first_half_maxheap.peek().unwrap();
                    if a >= t {
                        latter_half_minheap.push(-a);
                        latter_half_sum += a;
                    } else {
                        first_half_maxheap.pop();
                        latter_half_minheap.push(-t);
                        latter_half_sum += t;
                        first_half_maxheap.push(a);
                        first_half_sum -= t;
                        first_half_sum += a;
                    }
                } else {
                    let t = -latter_half_minheap.peek().unwrap();
                    if a <= t {
                        first_half_maxheap.push(a);
                        first_half_sum += a;
                    } else {
                        latter_half_minheap.pop();
                        latter_half_sum -= t;
                        latter_half_sum += a;
                        latter_half_minheap.push(-a);
                        first_half_maxheap.push(t);
                        first_half_sum += t;
                    }
                }
            }
        }
    }
}
