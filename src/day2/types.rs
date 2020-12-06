use std::str::FromStr;

pub struct Times(pub usize, pub usize);

pub struct Positions(pub usize, pub usize);

type Password = String;
type CharInPassword = char;

pub struct TimesCharPassword(pub Times, pub CharInPassword, pub Password);

pub struct PositionsCharPassword(pub Positions, pub CharInPassword, pub Password);

impl FromStr for Times {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_tuple: Vec<&str> = s.split("-").collect();
        if raw_tuple.len() != 2 {
            panic!(format!(
                "The string '{}' can not be converted into Times",
                s
            ))
        }

        let val1 = match raw_tuple[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s),
        };

        let val2 = match raw_tuple[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s),
        };

        Ok(Times(val1, val2))
    }
}

impl FromStr for Positions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_tuple: Vec<&str> = s.split("-").collect();
        if raw_tuple.len() != 2 {
            panic!(format!(
                "The string '{}' can not be converted into Positions",
                s
            ))
        }

        let val1 = match raw_tuple[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s),
        };

        let val2 = match raw_tuple[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("{} can not be converted into usize", s),
        };

        Ok(Positions(val1, val2))
    }
}

impl FromStr for TimesCharPassword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data: Vec<&str> = s.split_whitespace().collect();
        if raw_data.len() != 3 {
            panic!(format!(
                "The string '{}' can not be converted into TupleCharStr",
                s
            ))
        }

        let char: CharInPassword = match raw_data[1].chars().nth(0) {
            Some(c) => c,
            None => panic!(format!(
                "The string '{}' can not be converted into TupleCharStr",
                raw_data[1]
            )),
        };

        Ok(TimesCharPassword(
            Times::from_str(raw_data[0])?,
            char,
            Password::from(raw_data[2]),
        ))
    }
}

impl FromStr for PositionsCharPassword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_data: Vec<&str> = s.split_whitespace().collect();
        if raw_data.len() != 3 {
            panic!(format!(
                "The string '{}' can not be converted into TupleCharStr",
                s
            ))
        }

        let char: CharInPassword = match raw_data[1].chars().nth(0) {
            Some(c) => c,
            None => panic!(format!(
                "The string '{}' can not be converted into PositionsCharPassword",
                raw_data[1]
            )),
        };

        Ok(PositionsCharPassword(
            Positions::from_str(raw_data[0])?,
            char,
            Password::from(raw_data[2]),
        ))
    }
}
