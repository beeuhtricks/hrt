extern crate hrt;
extern crate chrono;

use hrt::Hrt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test = Hrt::new(args).unwrap();
    println!("{}", test);
}
