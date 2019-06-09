pub mod math;

/**
 * Split a number into vector of digits
 *
 * https://stackoverflow.com/questions/41536479/splitting-an-integer-into-individual-digits
 */
pub fn split_numbers(x: i32) -> Vec<i32> {
    fn x_inner(n: i32, xs: &mut Vec<i32>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs: Vec<i32> = Vec::new();
    x_inner(x, &mut xs);
    xs
}

pub fn is_in_sorted_vector(n: &i32, tab: Vec<i32>) -> bool {
    if tab.len() == 0 {
        return false;
    }

    let mid_index = tab.len() / 2;
    let mid = tab.get(mid_index).unwrap();

    if n == mid {
        true
    } else if n > mid {
        self::is_in_sorted_vector(n, tab[mid_index + 1..].to_vec())
    } else {
        self::is_in_sorted_vector(n, tab[..mid_index - 1].to_vec())
    }
}

// did not take the time to master generic data type traits & stuff
pub fn is_in_sorted_vector_64(n: &u64, tab: Vec<u64>) -> bool {
    if tab.len() == 0 {
        return false;
    }

    let mid_index = tab.len() / 2;
    let mid = tab.get(mid_index).unwrap();

    if n == mid {
        true
    } else if n > mid {
        self::is_in_sorted_vector_64(n, tab[mid_index + 1..].to_vec())
    } else {
        self::is_in_sorted_vector_64(n, tab[..mid_index].to_vec())
    }
}
