// Project Euler
// Problem 31
// https://projecteuler.net/problem=31

pub fn solve() {
    let mut total: u64 = 0;
    let mut ways: u64 = 0;

    let mut count: u64 = 0;
    let max_count: u64 = 2 * 3 * 5 * 11 * 21 * 41 * 101 * 201;

    // no brain brute force
    for p in 0..2 {
        for a in 0..3 {
            for b in 0..5 {
                for c in 0..11 {
                    for d in 0..21 {
                        for e in 0..41 {
                            for f in 0..101 {
                                for g in 0..201 {
                                    count += 1;
                                    total = p * 200
                                        + a * 100
                                        + b * 50
                                        + c * 20
                                        + d * 10
                                        + e * 5
                                        + f * 2
                                        + g;
                                    if total == 200 {
                                        ways += 1;
                                    }
                                    if count % 10000000 == 0 {
                                        println!(
                                            "{}/{} ({} %)",
                                            count,
                                            max_count,
                                            ((count as f32) / (max_count as f32) * (100 as f32))
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "There is {} ways to have £2 with the eight coins in circulation",
        ways
    );
}
