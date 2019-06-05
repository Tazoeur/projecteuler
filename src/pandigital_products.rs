// Project Euler
// Problem 32
// https://projecteuler.net/problem=32

use std::collections::HashMap;

pub fn solve() {
    let mut tab = HashMap::new();
    let mut product = 0;
    let mut number = String::new();
    let mut tuple: (u64, u64);

    let a_from = 1;
    let a_to = 100000;
    let b_from = 1;
    let b_to = 1000;

    let max_count: u64 = (a_to - a_from) * (b_to - b_from);

    let mut count: u64 = 0;

    for a in a_from..a_to {
        if contain_zero(a) {
            count += b_to - b_from;

            where_are_we(count, max_count);
            continue;
        }
        for b in b_from..b_to {
            count += 1;
            if contain_zero(b) {
                continue;
            }

            product = a * b;

            number = String::new();
            number.push_str(&a.to_string());
            number.push_str(&b.to_string());
            number.push_str(&product.to_string());

            for n in 1..10 {
                if is_pandigital(n, &number) {
                    // the concatenation of a, b and product is n-pandigital
                    tuple = get_ordered_tuple(a, b);
                    tab.insert(tuple, product);
                }
            }
        }
    }

    println!("There are {} products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.", tab.len());

    let mut sum = 0;
    for (tu, p) in &tab {
        sum += p;
    }

    println!("The sum of those products equals {}", sum);
}

// tell if the number i given as param contains at least one zero
fn contain_zero(i: u64) -> bool {
    let s: String = i.to_string();
    s.contains("0")
}

// tell if a number n written as a string is x-pandigital
pub fn is_pandigital(x: i32, n: &str) -> bool {
    if x < 1 || x > 9 {
        panic!("pandigital request must be pertinent!");
    }

    // if the lenght is different, it is not x-pandigital
    if n.len() != (x as usize) {
        return false;
    }

    // then check if foreach digit between 1 & x, it exists in the given number
    let mut p_str = String::new();
    for p in 1..(x + 1) {
        p_str = p.to_string();
        if !n.contains(&p_str) {
            return false;
        }
    }

    // if every tests was passed the number must be x-pandigital
    true
}

fn get_ordered_tuple(a: u64, b: u64) -> (u64, u64) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn get_ordered_triple_tuple(a: u64, b: u64, p: u64) -> (u64, u64, u64) {
    if a <= b && b <= p {
        (a, b, p)
    } else if a <= p && p <= b {
        (a, p, b)
    } else if b <= a && a <= p {
        (b, a, p)
    } else if b <= p && p <= a {
        (b, p, a)
    } else if p <= a && a <= b {
        (p, a, b)
    } else if p <= b && b <= a {
        (p, b, a)
    } else {
        panic!("there is a problem");
    }
}

fn where_are_we(c: u64, max: u64) {
    println!(
        "iteration {}/{} ({} %)",
        c,
        max,
        (c as f64) / (max as f64) * (100 as f64)
    );
}
