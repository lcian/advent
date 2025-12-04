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
    const N: usize = 140;
    let mut m = [[false; N]; N];
    for (i, line) in input.lines().enumerate() {
        for j in 0..line.len() {
            if line[j..j + 1].contains("@") {
                m[i][j] = true;
            }
        }
    }
    let mut res = 0;
    for i in 0..N {
        for j in 0..N {
            if !m[i][j] {
                continue;
            }
            let i = i as i32;
            let j = j as i32;
            let mut count = 0;
            for ii in i - 1..=i + 1 {
                for jj in j - 1..=j + 1 {
                    if ii < 0 || ii >= N as i32 || jj < 0 || jj >= N as i32 || (i == ii && j == jj)
                    {
                        continue;
                    }
                    if m[ii as usize][jj as usize] {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                res += 1;
            }
        }
    }
    println(res);
}

fn problem2() {
    let input = include_str!("example.txt");
    let mut res = 0;
    println(res);
}
