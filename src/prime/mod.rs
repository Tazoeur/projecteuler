use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

/**
 * Get the i th prime number
 *
 * 1 -> 2
 * 2 -> 3
 * 3 -> 5
 * 4 -> 7
 * 5 -> 11
 * ...
 */
pub fn get(i: u16) -> u64 {
    // if i not a right index, return zero
    if i < 1 {
        return 0;
    }

    let mut tab = self::get_stored_primes();
    let mut nbr = match tab.len() {
        0 => 1,
        n => tab[n - 1],
    };

    // while the tab array (filled with prime) does not have the size of the prime we want to get, continue the loop
    while tab.len() < usize::from(i) {
        nbr += 1;
        if self::is_prime(nbr, &tab) {
            self::add_prime_in_file(nbr, &mut tab);
        }
    }
    tab[usize::from(i) - 1]
}

/**
 * Infinite loop that calculate new primes and store them in the file
 */
pub fn calculate_prime() {
    let mut a;
    let mut i: u16 = (self::get_stored_primes().len() - 1) as u16;
    loop {
        i += 1;
        a = self::get(i);
        println!("prime #{} = {}", i, a);
    }
}

fn is_prime(nbr: u64, primes: &Vec<u64>) -> bool {
    // if the nbr can be devided by one of the existing primes,
    // it means that nbr is not a prime number
    for prime in primes {
        if nbr % prime == 0 {
            return false;
        }
    }
    true
}

/**
 * Return the path file name where are stored the calculated primes
 */
fn get_file_name() -> String {
    let filename = String::from("src/prime/calculated.prime");
    filename
}

/**
 * Get a vector filled with all the primes already calculated
 */
fn get_stored_primes() -> Vec<u64> {
    let file = File::open(self::get_file_name()).expect("error opening file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("error reading file");

    let mut primes: Vec<u64> = Vec::new();
    for lines in contents.split("\n") {
        for nbr in lines.split(",") {
            primes.push(nbr.trim().parse::<u64>().unwrap());
        }
    }

    primes
}

/**
 * Store the given prime at the end of the prime file
 */
fn add_prime_in_file(p: u64, primes: &mut Vec<u64>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(self::get_file_name())
        .unwrap();

    let before = if primes.len() % 100 == 0 { "\n" } else { ", " };
    let what_to_write = format!("{}{}", before.to_owned(), &p.to_string());

    // write!(file, "{}", String::from(eow) + p.to_string()).expect("error writing file");
    write!(file, "{}", what_to_write).expect("error writing file");

    primes.push(p);
}
