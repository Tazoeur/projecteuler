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
