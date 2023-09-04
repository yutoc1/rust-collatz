#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::stdin;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let n = 1000000;
    let mut acc = 0;
    for i in 1..=n {
        acc += collatz(i);
        acc %= 1000000007;
    }
    let end = start.elapsed();
    println!("実行時間: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
    println!("{}", acc);
}

fn collatz(mut i: usize) -> usize {
    let mut cnt = 0;
    while i != 1 {
        cnt += 1;
        if i % 2 == 0 {
            i /= 2;
        } else {
            i *= 3;
            i += 1;
        }
    }
    cnt
}
