use super::prime;
use std::thread;

pub fn solve() {
    let mut tab: Vec<(i64, i64)> = Vec::new();

    let mut n: u64 = 0;
    let mut eq: i64 = 0;
    let mut abs_eq: u64 = 0;
    let mut max_n: u64 = 0;
    let mut product = 0;

    let b_from = 0;
    let b_to = 1001;
    let a_from = -1000;
    let a_to = 1001;

    let max_parsing = (a_to - a_from) * (b_to - b_from);
    let mut parsing: f32 = 0.0;

    for b in b_from..b_to {
        if ((b as i32).abs() as u64) <= max_n || !prime::is_prime(&((b as i32).abs() as u64)) {
            parsing += (a_to - a_from) as f32;
            continue;
        }
        for a in a_from..a_to {
            parsing += 1.0;
            n = 0;
            loop {
                eq = (n.pow(2) as i64) + ((a as i64) * n as i64) + (b as i64);
                abs_eq = eq.abs() as u64;
                if !prime::is_prime(&abs_eq) {
                    break;
                }
                n += 1;
            }
            if product == 0 || n > max_n {
                max_n = n;
                product = a * b;
                if ((b as i32).abs() as u64) <= max_n {
                    parsing += (a_to - a) as f32;
                    break;
                }
            }
            if parsing as i32 % 100 == 0 {
                println!(
                    "done with calculus {}/{} ({}%)",
                    parsing,
                    max_parsing,
                    parsing / (max_parsing as f32) * (100 as f32)
                );
            }
        }
    }
    println!("final max n = {} and max product = {}", max_n, product);
}
