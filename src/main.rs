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

fn main() {
    let add5 = make_adder(5);
    let add10 = make_adder(10);

    println!("{}", add5(3));
    println!("{}", add10(3));

    let course = CourseConfig::default();

    greet();

    println!("Today: {}", OffsetDateTime::now_utc().date());
    println!("I am completing the course: {}", course);
    println!("My progress in the current module is:");
    show_progress(9, 14);
}
