// pub mod solution;

use solution::{is_leap_year, hello};

fn main() {
    println!("{}", hello());    
    println!("Is 1900 a leap year? {}", is_leap_year(1900));
}
