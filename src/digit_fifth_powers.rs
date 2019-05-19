// Problem 30
use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::ContinuousView;

pub fn solve() {
    // let a = super::utils::split_numbers(12345);
    let mut i = 10;
    loop {
        i += 1;
        if is_number_equals_to_sum_of_fifth_powers_of_their_digit(i) {
            println!("{} is good", i);
        }
    }
}

fn is_number_equals_to_sum_of_fifth_powers_of_their_digit(i: i32) -> bool {
    let tab = super::utils::split_numbers(i);
    let mut sum = 0;
    for x in tab {
        sum += x.pow(5);
    }
    sum == i
}

fn draw_graph() {
    let mut data: Vec<(f64, f64)> = Vec::new();
    for i in 2..100000 {
        data.push((
            (i as f64),
            self::get_difference_between_number_and_sum_of_fifth_powers_of_their_digit(i as i32),
        ));
    }

    let s1 = Scatter::from_slice(&data).style(
        scatter::Style::new()
            .marker(Marker::Square)
            .colour("#DD3355"),
    );
    let v = ContinuousView::new()
        .add(&s1)
        .x_range(1., 100000.)
        .y_range(-20000., 250000.)
        .x_label("The number")
        .y_label("The difference between the number and the sum of fifth powers of their digits");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg");
}

fn get_difference_between_number_and_sum_of_fifth_powers_of_their_digit(i: i32) -> f64 {
    let tab = super::utils::split_numbers(i);
    let mut sum = 0;
    for x in tab {
        sum += x.pow(5);
    }
    (sum - i) as f64
}
