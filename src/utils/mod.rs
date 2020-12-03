use std::str::FromStr;
use std::fs::File;
use std::io::Read;

pub fn get_input_by_lines<T: FromStr>(name: &str) -> Result<Vec<T>, T::Err> {
    let mut input = String::new();
    let _read_result = File::open(name)
        .unwrap()
        .read_to_string(&mut input);

    input.lines().map(T::from_str).collect()
}

pub fn get_input<T: FromStr>(name: &str) -> Result<T, T::Err> {
    let mut input = String::new();
    let _read_result = File::open(name)
        .unwrap()
        .read_to_string(&mut input);

    T::from_str(&input)
}