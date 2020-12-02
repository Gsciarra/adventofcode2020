use std::str::FromStr;

pub fn str_contain_char_a_range_of_times(times_char_password: &TimesCharPassword) -> bool {
    let TimesCharPassword(times, char, password) = times_char_password;
    let occurrences = password.chars().filter(|c| c == char).count();
    times.0 <= occurrences && occurrences <= times.1
}

pub fn str_contain_char_in_one_positions(positions_char_password: &PositionsCharPassword) -> bool {
    let PositionsCharPassword(positions, char, password) = positions_char_password;
    let is_in_position1 = password.chars().nth(positions.0 - 1).unwrap() == *char;
    if password.len() >= positions.1 {
        let is_in_position2 = password.chars().nth(positions.1 - 1).unwrap() == *char;
        (is_in_position1 && !is_in_position2) || (!is_in_position1 && is_in_position2)
    } else {
        is_in_position1
    }
}

struct Times(usize, usize);

pub struct Positions(usize, usize);

type Password = String;
type CharInPassword = char;

pub struct TimesCharPassword(Times, CharInPassword, Password);

pub struct PositionsCharPassword(Positions, CharInPassword, Password);

impl FromStr for Times {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_tuple: Vec<&str> = s.split("-").collect();
        if raw_tuple.len() != 2 {
            panic!(format!("The string '{}' can not be converted into Times", s))
        }

        let val1 = match raw_tuple[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s)
        };

        let val2 = match raw_tuple[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s)
        };

        Ok(Times(val1, val2))
    }
}

impl FromStr for Positions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_tuple: Vec<&str> = s.split("-").collect();
        if raw_tuple.len() != 2 {
            panic!(format!("The string '{}' can not be converted into Positions", s))
        }

        let val1 = match raw_tuple[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s)
        };

        let val2 = match raw_tuple[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s)
        };

        Ok(Positions(val1, val2))
    }
}

impl FromStr for TimesCharPassword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data: Vec<&str> = s.split_whitespace().collect();
        if raw_data.len() != 3 {
            panic!(format!("The string '{}' can not be converted into TupleCharStr", s))
        }

        let char: CharInPassword = match raw_data[1].chars().nth(0) {
            Some(c) => { c }
            None => { panic!(format!("The string '{}' can not be converted into TupleCharStr", raw_data[1])) }
        };

        Ok(
            TimesCharPassword(
                Times::from_str(raw_data[0])?,
                char,
                Password::from(raw_data[2]),
            )
        )
    }
}

impl FromStr for PositionsCharPassword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data: Vec<&str> = s.split_whitespace().collect();
        if raw_data.len() != 3 {
            panic!(format!("The string '{}' can not be converted into TupleCharStr", s))
        }

        let char: CharInPassword = match raw_data[1].chars().nth(0) {
            Some(c) => { c }
            None => { panic!(format!("The string '{}' can not be converted into PositionsCharPassword", raw_data[1])) }
        };

        Ok(
            PositionsCharPassword(
                Positions::from_str(raw_data[0])?,
                char,
                Password::from(raw_data[2]),
            )
        )
    }
}
