#![allow(dead_code)]

extern crate lib_euler;
use std::time::Instant;

fn main() {
    // start the timer
    let now = Instant::now();

    lib_euler::pandigital_products::solve();

    // display ping-pong informations
    println!(
        "Done after {}s ({}h)",
        now.elapsed().as_secs(),
        (now.elapsed().as_secs() as f64) / (3600 as f64)
    )
}
