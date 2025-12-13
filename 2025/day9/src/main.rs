#![no_std]
#![no_main]

use prelude::*;
prelude!();

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    //const N: usize = 8;
    const N: usize = 496;
    let input = include_str!("input.txt");
    let mut points: [(u64, u64); N] = [(0, 0); N];
    for (i, line) in input.lines().enumerate() {
        let (a, b) = line.trim_end_matches("\n").split_once(",").unwrap();
        points[i] = (atoi(a), atoi(b));
    }
    let mut res = 0;
    for i in 0..N {
        for j in i + 1..N {
            let a = points[i];
            let b = points[j];
            let width = (a.0 as i64 - b.0 as i64).abs() + 1;
            let height = (a.1 as i64 - b.1 as i64).abs() + 1;
            res = res.max(width * height);
        }
    }
    println(res);
}

fn problem2() {
    const N: usize = 8;
    let input = include_str!("example.txt");
    let mut res = 0;
    let mut ranges: [u64; N] = [0; N];
    println(res);
}
