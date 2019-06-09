/**
 * @return n!
 */
pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut f: u64 = 1;

    for i in 1..(n + 1) {
        f *= i;
    }
    f
}

/**
 * @return the number of combination possible when taking p elements from n available elements
 * A combination is an arrangement without repetition (order does not matter)
 */
pub fn combinatorics_combination_count(p: i32, n: i32) -> u64 {
    // n negative or trying to have an combination of more element than there is available
    if p < 0 || p > n {
        return 0;
    }

    // if trying to get zero element from the array, there is only one way to do it
    if p == 0 {
        return 1;
    }

    // if trying to get all element from the array, there is only one way to do it
    if p == n {
        return 1;
    }

    // if trying to get one element from the array, there is as many way to do it as there is element in the array
    if p == 1 {
        return n as u64;
    }

    // else, n!/(n-p)!
    factorial(n as u64) / (factorial(p as u64) * factorial((n as u64) - (p as u64)))
}

/**
 * @requires p > 0
 * @return all the combinations of p elements from the elements bag
 *          There will be n combinations, with n = combinatorics_combination_count(p, elements.len())
 *
 *          ex : let p = 2 and elements = [1, 2, 3, 4, 5]; the result will be
 *          [[1, 2], [1, 3], [1, 4], [1, 5], [2, 3], [2, 4], [2, 5], [3, 4], [3, 5], [4, 5]]
 */
pub fn combinatorics_combination<T>(p: i32, elements: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut tab: Vec<Vec<T>> = Vec::new();

    // simplest case, if we want to get the combinations of 1 element from the elements bag,
    // there is elements.len() differents Vec with a lenght of 1
    if p == 1 {
        for e in elements {
            let mut t: Vec<T> = Vec::with_capacity(p as usize);
            t.push((*e).clone());
            tab.push(t);
        }
        return tab;
    }

    let mut count = 0;
    // we go through the elements of the bag
    for e in elements {
        count += 1;

        // and for each one, we generate the combinations of a subset of the elements bag
        let sub_comb: Vec<Vec<T>> = combinatorics_combination(p - 1, &elements[count..]);

        for sc in sub_comb {
            // and we merge the element with the subset's combinations that have just been generated
            let mut t: Vec<T> = Vec::with_capacity(p as usize);
            t.push((*e).clone());
            for s in sc {
                t.push(s);
            }

            // finaly we add this combination to the `tab` vector
            tab.push(t);
        }
    }
    tab
}

/**
 * @requires p > 0
 * @return all the permutation of p elements from the elements bag
 *          There will be n permutation, with n = combinatorics_permutation_count(p, elements.len())
 *
 *          ex : let p = 2 and elements = [1, 2, 3, 4, 5]; the result will be
 *          [[1, 2], [2, 1], [1, 3], [3, 1], [1, 4], [4, 1], [1, 5], [5, 1], [2, 3], [3, 2],
 *           [2, 4], [4, 2], [2, 5], [5, 2], [3, 4], [4, 3], [3, 5], [5, 3], [4, 5], [5, 4]]
 */
pub fn combinatorics_permutation<T>(p: i32, elements: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut tab: Vec<Vec<T>> = Vec::new();
    let mut pos: usize;

    // simplest case, if we want to get the permutation of 1 element from the elements bag,
    // there is elements.len() differents Vec with a lenght of 1
    if p == 1 {
        for e in elements {
            let mut t: Vec<T> = Vec::with_capacity(p as usize);
            t.push((*e).clone());
            tab.push(t);
        }
        return tab;
    }

    let mut count = 0;
    // we go through the elements of the bag
    for e in elements {
        count += 1;

        // and for each one, we generate the permutations of a subset of the elements bag
        let sub_perm: Vec<Vec<T>> = combinatorics_permutation(p - 1, &elements[count..]);

        for sc in sub_perm {
            for position in 0..(p as usize) {
                let mut t: Vec<T> = Vec::with_capacity(p as usize);
                pos = 0;
                for s in &sc {
                    if pos == position {
                        t.push((*e).clone());
                    }
                    t.push((*s).clone());
                    pos += 1;
                }
                if pos == position {
                    t.push((*e).clone());
                }
                tab.push(t);
            }
        }
    }
    tab
}
