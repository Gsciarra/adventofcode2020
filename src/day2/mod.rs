use std::str::FromStr;
use std::iter::FromIterator;
use std::slice::Iter;
use std::char::ParseCharError;

pub fn str_contain_char_a_range_of_times(times_char_password: &TimesCharPassword) -> bool {
    let TimesCharPassword(times, char, password) = times_char_password;
    let occurrences = password.chars().filter(|c| c == char).count();
    times.0 <= occurrences && occurrences <= times.1
}

pub fn str_contain_char_in_one_positions(str: &Password, char: &CharInPassword, positions: &Positions) -> bool {
    let is_in_position1 = str.chars().nth(positions.0 - 1).unwrap() == *char;
    if str.len() >= positions.1 {
        let is_in_position2 = str.chars().nth(positions.1 - 1).unwrap() == *char;
        (is_in_position1 && !is_in_position2) || (!is_in_position1 && is_in_position2)
    } else {
        is_in_position1
    }
}

struct Times(usize, usize);

pub struct Positions(usize, usize);

type Password = String;
type CharInPassword = char;

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

// impl FromStr for CharInPassword {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.chars().nth(0) {
//             Some(c) => { Ok(c) }
//             None => { panic!(format!("The string '{}' can not be converted into Char", s)) }
//         }
//     }
// }


pub struct TimesCharPassword(Times, CharInPassword, Password);

pub struct PositionsCharPassword(Positions, CharInPassword, Password);

// pub struct TimesCharPasswordCollection(Vec<TimesCharPassword>);


impl FromStr for TimesCharPassword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data: Vec<&str> = s.split_whitespace().collect();
        if raw_data.len() != 3 {
            panic!(format!("The string '{}' can not be converted into TupleCharStr", s))
        }

        let char = match CharInPassword::from_str(raw_data[1]) {
            Ok(c) => { c }
            Err(_) => { panic!(format!("The string '{}' can not be converted into TupleCharStr", s)) }
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

        let char = match CharInPassword::from_str(raw_data[1]) {
            Ok(c) => { c }
            Err(_) => { panic!(format!("The string '{}' can not be converted into TupleCharStr", s)) }
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

// impl FromIterator<&str> for Vec<TupleCharStr> {
//     fn from_iter<T: IntoIterator<Item=&'static str>>(iter: T) -> Self {
//         let mut result: Vec<TupleCharStr> = vec![];
//
//         for element in iter {
//             match TupleCharStr::from_str(&element) {
//                 Ok(item) => { result.push(item) }
//                 Err(_) => panic!("Invalid element: {}", element)
//             }
//         }
//
//         result
//     }
// }

// impl TimesCharPasswordCollection {
//     fn new() -> TimesCharPasswordCollection {
//         TimesCharPasswordCollection(Vec::new())
//     }
//
//     fn add(&mut self, elem: TimesCharPassword) {
//         self.0.push(elem);
//     }
//
//     pub fn iter(&mut self) -> Iter<'_, TimesCharPassword> {
//         self.0.iter()
//     }
// }

// impl<'a> FromIterator<&'a str> for TimesCharPasswordCollection {
//     fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
//         let mut result = TimesCharPasswordCollection::new();
//
//         for i in iter {
//             match TimesCharPassword::from_str(i) {
//                 Ok(item) => { result.add(item.clone()) }
//                 Err(_) => {}
//             }
//         }
//
//         result
//     }
// }