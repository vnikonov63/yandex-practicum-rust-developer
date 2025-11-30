mod config;
mod utils;

use std::error::Error;
use std::fmt;

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

// Exercise from 0.3.3.1
fn get_port_config(env_var: Option<String>) -> u16 {
    env_var.and_then(|s| s.parse().ok()).unwrap_or(8080)
}

// Exercise from 0.3.3.2
#[derive(Debug)]
enum AuthError {
    InvalidPassword,
    UserNotFound(String),
    TokenExpired,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidPassword => write!(f, "Wrong password"),
            AuthError::UserNotFound(name) => write!(f, "User '{}' could not be found", name),
            AuthError::TokenExpired => write!(f, "Token expired"),
        }
    }
}

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AuthError::InvalidPassword => None,
            AuthError::UserNotFound(_) => None,
            AuthError::TokenExpired => None,
        }
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

    // Exercise from 0.3.3.1
    assert_eq!(get_port_config(Some("3000".to_string())), 3000);
    assert_eq!(get_port_config(Some("abc".to_string())), 8080);
    assert_eq!(get_port_config(None), 8080);

    // Exercise from 0.3.3.2
    let err = AuthError::UserNotFound("john".to_string());
    assert_eq!(err.to_string(), "User 'john' could not be found");

    let err = AuthError::InvalidPassword;
    assert_eq!(err.to_string(), "Wrong password");

    let err = AuthError::TokenExpired;
    assert_eq!(err.to_string(), "Token expired");

    // Intro Stuff
    let course = CourseConfig::default();
    greet();
    println!("Today: {}", OffsetDateTime::now_utc().date());
    println!("I am completing the course: {}", course);
    println!("My progress in the current module is:");
    show_progress(9, 14);
}
