mod prime;
mod utils;

pub mod digit_fifth_powers;

/**
 * Access to private function of prime module
 */
pub fn prime_loop() {
    prime::calculate_prime();
}
