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
    let mut pos: i64 = 50;

    for line in input.lines() {
        let (sign, count) = line.split_at(1);
        let sign: i64 = match sign {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };
        let count: u64 = atoi(count);
        pos = pos + (sign * count as i64);
        if pos < 0 {
            pos = 100 + pos;
        }
        pos = pos % 100;

        if pos == 0 {
            res += 1;
        }
    }
    println(res as u64);
}

fn problem2() {
    let input = include_str!("input.txt");
    let mut res = 0;
    let mut pos: i64 = 50;

    for line in input.lines() {
        let (sign, count) = line.split_at(1);
        let mut count: u64 = atoi(count);
        match sign {
            "L" => {
                while count > 0 {
                    let c = count.min(100);
                    count -= c;

                    let old_pos: i64 = pos as i64;
                    pos -= c as i64;
                    if (old_pos > 0 && pos <= 0) || (pos == -100) {
                        res += 1;
                    }
                    if pos < 0 {
                        pos = 100 + pos;
                    }
                }
            }
            "R" => {
                while count > 0 {
                    let c = count.min(100);
                    count -= c;

                    pos += c as i64;
                    if pos >= 100 {
                        pos = pos % 100;
                        res += 1;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    println(res as u64);
}
