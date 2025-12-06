#![no_std]
#![no_main]

use core::cmp::Ordering;

use prelude::*;
prelude!();

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    const N: usize = 192;
    let input = include_str!("input.txt");
    let mut res = 0;
    let mut ranges: [(u64, u64); N] = [(0, 0); N];
    for (i, line) in input.lines().enumerate() {
        let line = line.trim_end_matches("\n");
        if i == N {
            continue;
        }
        if i < N {
            let (a, b) = line.split_once("-").unwrap();
            let a = atoi(a);
            let b = atoi(b);
            ranges[i] = (a, b);
        } else {
            let x = atoi(line);
            let mut fresh = false;
            for (a, b) in ranges {
                if x >= a && x <= b {
                    fresh = true;
                    break;
                }
            }
            if fresh {
                res += 1;
            }
        }
    }
    println(res);
}

fn sort(arr: &mut [(i64, i64)]) {
    for i in 0..arr.len() {
        let mut k = i;
        for j in (i + 1)..arr.len() {
            if arr[j].0 < arr[k].0 || (arr[k].0 == arr[j].0 && arr[j].1 < arr[k].1) {
                k = j;
            }
        }
        let tmp = arr[i];
        arr[i] = arr[k];
        arr[k] = tmp;
    }
}

fn problem2() {
    const N: usize = 192;
    let input = include_str!("input.txt");
    let mut ranges: [(i64, i64); N] = [(0, 0); N];
    for (i, line) in input.lines().enumerate() {
        let line = line.trim_end_matches("\n");
        if i < N {
            let (a, b) = line.split_once("-").unwrap();
            let a = atoi(a);
            let b = atoi(b);
            ranges[i] = (a as i64, b as i64);
            continue;
        }
        sort(&mut ranges);
        for i in 0..N {
            if ranges[i].0 == -1 {
                continue;
            }
            let mut end = ranges[i].1;
            for j in i + 1..N {
                if ranges[j].0 == -1 {
                    unreachable!();
                }
                let (a, b) = ranges[j];
                if a <= end {
                    end = b.max(end);
                    ranges[j] = (-1, -1);
                } else {
                    break;
                }
            }
            ranges[i].1 = end;
        }
        let res = ranges
            .iter()
            .filter(|(a, _)| *a != -1)
            .map(|(a, b)| (*a as u64, *b as u64))
            .fold(0u64, |acc, (a, b)| acc + b - a + 1);
        println(res);
        return;
    }
}
