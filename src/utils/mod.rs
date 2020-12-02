use std::str::FromStr;
use std::iter::FromIterator;
use std::fs::File;
use std::io::Read;

pub fn get_input<T: FromStr>(name: &str) -> Result<Vec<T>, T::Err> {
    let mut input = String::new();
    let _read_result = File::open(name)
        .unwrap()
        .read_to_string(&mut input);

    input.lines().map(T::from_str).collect()
}