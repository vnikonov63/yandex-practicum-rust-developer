mod config;
mod utils;

use config::CourseConfig;
use time::OffsetDateTime;
use utils::helpers::{greet, show_progress};

// TODO: Think of a better place to store the exercises, so I can have many main functions, e.t.c

// Exercise from 0.3.1
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + n
}

// Exercise from 0.3.2.1
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&n| n % 2 == 0).copied()
}

// Exercise from 0.3.2.2
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Exercise from 0.3.1
    let add5 = make_adder(5);
    let add10 = make_adder(10);

    println!("{}", add5(3));
    println!("{}", add10(3));

    // Exercise from 0.3.2.1
    assert_eq!(find_first_even(&[1, 3, 4, 7, 8]), Some(4));
    assert_eq!(find_first_even(&[1, 3, 5, 7]), None);

    // Exercise from 0.3.2.2
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));

    let course = CourseConfig::default();

    greet();

    println!("Today: {}", OffsetDateTime::now_utc().date());
    println!("I am completing the course: {}", course);
    println!("My progress in the current module is:");
    show_progress(9, 14);
}
