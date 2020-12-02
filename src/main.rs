mod utils;
mod day1;
mod day2;

use crate::day2::{
    TimesCharPassword,
    PositionsCharPassword,
};
use crate::utils::get_input;

fn main() {
    day1();
    day2();
}

fn day2() {
    let input1 = get_input::<TimesCharPassword>("input-day2.txt").unwrap();
    let input2 = get_input::<PositionsCharPassword>("input-day2.txt").unwrap();

    println!("Day 2 - part 1: {:?}", input1.into_iter().filter(|tcp| day2::str_contain_char_a_range_of_times(tcp)).count());
    println!("Day 2 - part 2: {:?}", input2.into_iter().filter(|pcp| day2::str_contain_char_in_one_positions(pcp)).count());
}

fn day1() {
    let input = get_input::<usize>("input-day1.txt").unwrap();
    let combination_test1 = day1::find_combination_to_obtain_n(&input, 2, 2020);
    let combination_test2 = day1::find_combination_to_obtain_n(&input, 3, 2020);

    println!("Day 1 - part 1: {:?}", combination_test1.unwrap().iter().fold(1, |acc, x| acc * x));
    println!("Day 1 - part 2: {:?}", combination_test2.unwrap().iter().fold(1, |acc, x| acc * x));
}


