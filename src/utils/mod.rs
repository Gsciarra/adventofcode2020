pub mod area_map;

use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn get_input_by_lines<T: FromStr>(name: &str) -> Result<Vec<T>, T::Err> {
    let mut input = String::new();
    let _read_result = File::open(name).unwrap().read_to_string(&mut input);

    input.lines().map(T::from_str).collect()
}

pub fn get_input<T: FromStr>(name: &str) -> Result<T, T::Err> {
    let mut input = String::new();
    let _read_result = File::open(name).unwrap().read_to_string(&mut input);

    T::from_str(&input)
}

pub fn print_solution<T: Debug>(day: usize, step: usize, result: T) {
    println!("Day {} - part {}: {:?}", day, step, result);
}
