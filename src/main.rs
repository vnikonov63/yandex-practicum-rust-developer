mod config;
mod utils;

use config::CourseConfig;
use time::OffsetDateTime;
use utils::helpers::{greet, show_progress};

fn main() {
    let course = CourseConfig::default();

    greet();

    println!("Today: {}", OffsetDateTime::now_utc().date());
    println!("I am completing the course: {}", course);
    println!("My progress in the current module is:");
    show_progress(9, 14);
}
