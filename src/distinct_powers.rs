// problem 29
use super::hugeint::HI;
use std::collections::HashSet;

pub fn solve() {
    let mut set: HashSet<String> = HashSet::new();

    for a in 2..101 {
        for b in 2..101 {
            set.insert(self::custom_power(a as i32, b as u32));
        }
    }

    println!("There is {} elements in the set", set.len());
    // println!("{:?}", set);
}

fn custom_power(a: i32, b: u32) -> String {
    let a_hi = HI::new(a);

    a_hi.power(b).display()
}
