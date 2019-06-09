#![allow(dead_code)]
mod prime;
pub mod utils;

pub mod hugeint;

pub mod coin_sums;
pub mod digit_fifth_powers;
pub mod distinct_powers;
pub mod pandigital_products;
pub mod quadratic_primes;

/**
 * Access to private function of prime module
 */
pub fn prime_loop() {
    prime::calculate_prime();
}
