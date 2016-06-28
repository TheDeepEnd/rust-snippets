// extern crate time;
//
// use std::time::Duration;

use std::time::{Duration, SystemTime};

fn main() {

let now = SystemTime::now();
while true
{
println!("{}", now.elapsed().as_secs());
}
    // let now = time::get_time();
    // println!("{}", now);
    // let later = now + Duration::minutes(5);
    // println!("later: {}", later);
}
