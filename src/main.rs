mod utils;
mod day2;

use crate::day2::{TimesCharPassword};

fn main() {
    day2();
}

fn day2() {
    let mut input = utils::get_input::<TimesCharPassword>("input-day2.txt").unwrap();

    // let parsed_data: Vec<((usize, usize), char, &str)> = input.lines().map(|line| {
    //     let raw_data: Vec<&str> = line.split_whitespace().collect();
    //     let raw_range: Vec<&str> = raw_data[0].split("-").collect();
    //     let range: (usize, usize) = (raw_range[0].parse().unwrap(), raw_range[1].parse().unwrap());
    //     let char = raw_data[1].chars().nth(0).unwrap();
    //     let password = raw_data[2];
    //
    //     (range, char, password)
    // }).collect();

    println!("{:?}", input.into_iter().filter(|tcp| day2::str_contain_char_a_range_of_times(tcp)).count());
}

// fn day1() {
//     let input = get_input("input-day1.txt");
//
//     let numbers: Vec<usize> = input
//         .lines()
//         .map(|s| s.parse::<usize>().unwrap())
//         .collect();
//
//     let combination = find_combination_to_obtain_n(&numbers, 3, 2020);
//
//     println!("input: {:?}", numbers);
//     println!("combination: {:?}", combination);
//     println!("result: {:?}", combination.unwrap().iter().fold(1, |acc, x| acc * x));
// }


