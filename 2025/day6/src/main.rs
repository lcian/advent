#![no_std]
#![no_main]

use prelude::*;
prelude!();

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    //const N: usize = 4;
    //let input = include_str!("example.txt");
    const N: usize = 1000;
    let input = include_str!("input.txt");
    let mut ops = [""; N];
    let mut nums = [0u64; N];
    for (i, line) in input.lines().rev().enumerate() {
        let line = line.trim_end_matches("\n");
        if i == 0 {
            let mut j = 0;
            for entry in line.split(" ") {
                if !entry.is_empty() {
                    ops[j] = entry;
                    j += 1;
                }
            }
            continue;
        }
        if i == 1 {
            let mut j = 0;
            for entry in line.split(" ") {
                if !entry.is_empty() {
                    nums[j] = atoi(entry);
                    j += 1;
                }
            }
            continue;
        }
        let mut j = 0;
        for entry in line.split(" ") {
            if !entry.is_empty() {
                nums[j] = match ops[j] {
                    "+" => nums[j] + atoi(entry),
                    "*" => nums[j] * atoi(entry),
                    _ => unreachable!(),
                };
                j += 1;
            }
        }
    }
    println(nums.iter().sum::<u64>());
}

fn problem2() {
    //const M: usize = 3;
    //const N: usize = 15;
    //let input = include_str!("example.txt");
    const M: usize = 4;
    const N: usize = 3736;
    let input = include_str!("input.txt");

    let mut lines = [""; M];
    for (i, line) in input.lines().take(M).enumerate() {
        lines[i] = line.trim_end_matches("\n");
    }

    let mut ops = [""; N];
    let mut i = 0;
    for entry in input.lines().last().unwrap().split(" ") {
        if !entry.is_empty() {
            ops[i] = entry;
            i += 1;
        }
    }

    let mut nums = [0u64; N];
    let mut k = 0;
    for j in 0..N {
        let mut num = 0;
        for i in 0..M {
            let c = lines[i].chars().skip(j).take(1).last().unwrap();
            if c == ' ' {
                continue;
            }
            let c = c as u64 - 48;
            num = num * 10 + c;
        }
        if num == 0 {
            k += 1;
            continue;
        }
        nums[k] = match ops[k] {
            "+" => nums[k] + num,
            "*" => {
                if nums[k] == 0 {
                    num
                } else {
                    nums[k] * num
                }
            }
            _ => unreachable!(),
        };
    }
    println(nums.iter().sum::<u64>());
}
