use libprocon::*;
use std::collections::HashSet;

fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut a = a;
    let mut A = vec![0];
    A.append(&mut a);

    let mut ball_boxes = HashSet::new();
    for i in (1..=N).rev() {
        let mut mul = i;
        let mut times = 2;
        let mut box_count = 0;
        while mul <= N {
            if ball_boxes.contains(&mul) {
                box_count += 1;
            }
            mul = i * times;
            times += 1;
        }

        let aa = A[i];
        if (box_count % 2 == 0 && aa % 2 != 0) || (box_count % 2 != 0 && aa % 2 == 0) {
            ball_boxes.insert(i);
        }
    }
    println!("{}", ball_boxes.len());
    if !ball_boxes.is_empty() {
        println!(
            "{}",
            ball_boxes
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
