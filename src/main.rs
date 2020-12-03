mod utils;
mod day1;
mod day2;
mod day3;

use crate::day2::{
    TimesCharPassword,
    PositionsCharPassword,
};

use crate::day3::{LocationMap, Steps};
use crate::utils::{get_input_by_lines, get_input};

fn main() {
    day1();
    day2();
    day3();
}

fn day3() {
    let location_map = get_input::<LocationMap>("input-day3.txt").unwrap();
    let paths: Vec<Steps> = vec![Steps(1, 1), Steps(3, 1), Steps(5, 1), Steps(7, 1), Steps(1, 2)];

    println!("Day 3 - part 1: {:?}", day3::count_trees_on_path(&location_map, &Steps(3, 1)));
    println!("Day 3 - part 2: {:?}", paths.iter().fold(1, |acc, steps| acc * day3::count_trees_on_path(&location_map, steps)));
}

fn day2() {
    let input1 = get_input_by_lines::<TimesCharPassword>("input-day2.txt").unwrap();
    let input2 = get_input_by_lines::<PositionsCharPassword>("input-day2.txt").unwrap();

    println!("Day 2 - part 1: {:?}", input1.into_iter().filter(|tcp| day2::str_contain_char_a_range_of_times(tcp)).count());
    println!("Day 2 - part 2: {:?}", input2.into_iter().filter(|pcp| day2::str_contain_char_in_one_positions(pcp)).count());
}

fn day1() {
    let input = get_input_by_lines::<usize>("input-day1.txt").unwrap();
    let combination_test1 = day1::find_combination_to_obtain_n(&input, 2, 2020);
    let combination_test2 = day1::find_combination_to_obtain_n(&input, 3, 2020);

    println!("Day 1 - part 1: {:?}", combination_test1.unwrap().iter().fold(1, |acc, x| acc * x));
    println!("Day 1 - part 2: {:?}", combination_test2.unwrap().iter().fold(1, |acc, x| acc * x));
}


