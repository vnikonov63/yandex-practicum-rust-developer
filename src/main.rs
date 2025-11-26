mod config;
mod utils;

//use config::DEFAULT_COURSE_NAME;
//use time::OffsetDateTime;
//use utils::helpers::greet;
use utils::helpers::show_progress;

fn main() {
    //greet();
    //println!("Today: {}", OffsetDateTime::now_utc().date());
    //println!("I am completing the course: {}", DEFAULT_COURSE_NAME);

    println!("My progress in the current module:");

    show_progress(5, 15);
}
