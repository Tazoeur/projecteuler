#[derive(Debug)]
pub struct HI {
    value: Vec<i32>,
    is_positive: bool,
}

impl HI {
    pub fn new(i: i32) -> HI {
        let mut tmp = split_numbers(i);
        tmp.reverse();
        HI {
            value: tmp,
            is_positive: i >= 0,
        }
    }

    pub fn display(&self) -> String {
        let mut display = String::new();
        for i in &self.value {
            display = format!("{}{}", i, display);
        }
        display
    }

    pub fn add(&self, n: &HI) -> HI {
        let max_size = std::cmp::max(self.value.len(), n.value.len());
        let mut final_result: Vec<i32> = Vec::new();
        let mut result = 0;
        let mut report = 0;

        let mut self_value = 0;
        let mut n_value = 0;

        for i in 0..max_size {
            self_value = match self.value.get(i) {
                None => 0,
                Some(&n) => n,
            };
            n_value = match n.value.get(i) {
                None => 0,
                Some(&n) => n,
            };

            result = self_value + n_value + report;
            report = 0;

            while result >= 10 {
                result -= 10;
                report += 1;
            }
            final_result.push(result);
        }

        if report != 0 {
            final_result.push(report);
            report = 0;
        }

        HI {
            value: final_result,
            is_positive: true,
        }
    }

    pub fn multiply(&self, n: &HI) -> HI {
        let mut final_hi: HI = HI::new(0);
        let mut tmp_hi: HI;

        for i in 0..self.value.len() {
            let mut nn = HI::new(0);
            let value: i32 = match self.value.get(i) {
                None => 0,
                Some(&n) => n,
            };

            for v in 0..value {
                nn = nn.add(&n);
            }
            let mut tmp_val = nn.value;
            for j in 0..i {
                tmp_val.insert(0, 0);
            }
            tmp_hi = HI {
                value: tmp_val,
                is_positive: true,
            };
            final_hi = final_hi.add(&tmp_hi);
        }

        final_hi
    }

    pub fn power(&self, n: u32) -> HI {
        let mut final_hi = self.clone();

        let value = self.clone();
        for i in 1..n {
            final_hi = final_hi.multiply(&value);
        }

        final_hi
    }

    pub fn clone(&self) -> HI {
        HI {
            value: self.value.clone(),
            is_positive: self.is_positive,
        }
    }
}

/**
 * Split a number into vector of digits
 *
 * https://stackoverflow.com/questions/41536479/splitting-an-integer-into-individual-digits
 */
fn split_numbers(x: i32) -> Vec<i32> {
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
