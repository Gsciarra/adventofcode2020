mod types;

use crate::day2::types::{PositionsCharPassword, TimesCharPassword};
use crate::utils::{get_input_by_lines, print_solution};
use std::str::FromStr;

fn str_contain_char_a_range_of_times(times_char_password: &TimesCharPassword) -> bool {
    let TimesCharPassword(times, char, password) = times_char_password;
    let occurrences = password.chars().filter(|c| c == char).count();
    times.0 <= occurrences && occurrences <= times.1
}

fn str_contain_char_in_one_positions(positions_char_password: &PositionsCharPassword) -> bool {
    let PositionsCharPassword(positions, char, password) = positions_char_password;
    let is_in_position1 = password.chars().nth(positions.0 - 1).unwrap() == *char;
    if password.len() >= positions.1 {
        let is_in_position2 = password.chars().nth(positions.1 - 1).unwrap() == *char;
        (is_in_position1 && !is_in_position2) || (!is_in_position1 && is_in_position2)
    } else {
        is_in_position1
    }
}

pub fn solution1() {
    let input = get_input_by_lines::<TimesCharPassword>("input-day2.txt").unwrap();

    print_solution(
        2,
        1,
        input
            .into_iter()
            .filter(|tcp| str_contain_char_a_range_of_times(tcp))
            .count(),
    );
}

pub fn solution2() {
    let input = get_input_by_lines::<PositionsCharPassword>("input-day2.txt").unwrap();

    print_solution(
        2,
        2,
        input
            .into_iter()
            .filter(|pcp| str_contain_char_in_one_positions(pcp))
            .count(),
    );
}
