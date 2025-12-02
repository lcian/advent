#![no_std]
#![no_main]

use prelude::*;
prelude!();

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let input = include_str!("input.txt");
    let mut res = 0;
    for range in input.trim_end_matches("\n").split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let a = atoi(a);
        let b = atoi(b);
        for x in a..=b {
            let tmp = itoa(x);
            let tmp = str::from_utf8(&tmp).unwrap();
            let s = tmp.trim_start_matches("0");
            let m = s.len() / 2;
            if m * 2 != s.len() {
                continue;
            }
            let (left, right) = s.split_at(m);
            if left == right {
                res += x;
            }
        }
    }
    println(res);
}

fn problem2() {
    let input = include_str!("input.txt");
    let mut res = 0;
    for range in input.trim_end_matches("\n").split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let a = atoi(a);
        let b = atoi(b);
        for x in a..=b {
            let tmp = itoa(x);
            let tmp = str::from_utf8(&tmp).unwrap();
            let s = tmp.trim_start_matches("0");
            for i in 1..=(s.len() / 2) {
                if s.len() % i != 0 {
                    continue;
                }
                let part = s.split_at(i).0;
                let mut ok = true;
                for rem in s.split(part) {
                    if rem.len() > 0 {
                        ok = false;
                    }
                }
                if ok {
                    res += x;
                    break;
                }
            }
        }
    }
    println(res);
}
