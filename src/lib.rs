#![allow(dead_code)]
mod prime;
mod utils;

pub mod hugeint;

pub mod digit_fifth_powers;
pub mod distinct_powers;
pub mod quadratic_primes;

/**
 * Access to private function of prime module
 */
pub fn prime_loop() {
    prime::calculate_prime();
}
