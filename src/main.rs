#![allow(dead_code)]

extern crate lib_euler;
use lib_euler::utils::math;
use std::time::Instant;

fn main() {
    // start the timer
    let now = Instant::now();

    lib_euler::pandigital_products::solve_alt();

    // let numbers = [1, 2, 3, 4, 5];
    // let test = math::combinatorics_permutation(5, &numbers);
    // println!("test = {:?}", test);

    // display ping-pong informations
    println!(
        "Done after {}s ({}h)",
        now.elapsed().as_secs(),
        (now.elapsed().as_secs() as f64) / (3600 as f64)
    )
}
