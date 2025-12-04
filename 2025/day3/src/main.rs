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
    for line in input.lines() {
        let line = line.trim_end_matches("\n");
        let mut max = 0;
        for a in 1..=9 {
            let c = itoa(a);
            let c = str::from_utf8(&c).unwrap().trim_start_matches("0");
            let Some((_, rest)) = line.split_once(c) else {
                continue;
            };
            let Some(b) = rest.chars().map(|c| c.to_digit(10).unwrap()).max() else {
                continue;
            };
            let curr = a * 10 + b as u64;
            max = curr.max(max);
        }
        res += max;
    }
    println(res);
}

fn problem2() {
    const M: usize = 100;
    let input = include_str!("input.txt");
    let mut res: u64 = 0;
    for line in input.lines() {
        let line = line.trim_end_matches("\n");
        let mut dp = [[0i64; M]; 13];
        for i in 1..=12 {
            for j in 0..M {
                if M - j < i {
                    dp[i][j] = i64::MIN;
                }
            }
        }
        for i in 1..=12 {
            for j in (0..M).rev() {
                let x = atoi(line.get(j..(j + 1)).unwrap()) as i64;
                if i == 1 {
                    dp[i][j] = dp[i][j].max(x);
                }
                for k in j + 1..M {
                    dp[i][j] = dp[i][j].max(dp[i][k]);
                    dp[i][j] = dp[i][j].max(x * (10i64.pow(i as u32 - 1)) + dp[i - 1][k]);
                }
            }
        }
        res += dp[12][0].abs() as u64;
    }
    println(res);
}
