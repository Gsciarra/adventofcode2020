use std::str::FromStr;
use crate::day3::Square::{Open, Tree};
use std::ops::Add;

pub fn count_trees_on_path(location_map: &LocationMap, steps: &Steps) -> usize {
    let mut current_position = Position(0, 0);

    let mut total_trees = 0;
    while current_position.1 < location_map.height {
        if location_map.is_there_a_tree(&current_position) {
            total_trees = total_trees + 1;
        }

        current_position = current_position + &steps;
    }

    total_trees
}

#[derive(Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

struct Position(usize, usize);

pub struct Steps(pub usize, pub usize);

pub struct LocationMap {
    map: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl LocationMap {
    fn is_there_a_tree(&self, position: &Position) -> bool {
        self.get_square(position) == &Tree
    }

    fn get_square(&self, position: &Position) -> &Square {
        &self.map[position.1][position.0 % self.width]
    }
}

impl FromStr for LocationMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let location_map: Vec<Vec<Square>> = s.lines()
            .map(|line| line.chars().map(|c| Square::from_char(c)).collect())
            .collect();

        Ok(LocationMap {
            width: location_map[0].len(),
            height: location_map.len(),
            map: location_map,
        })
    }
}

impl Square {
    fn from_char(c: char) -> Self {
        if c == '.' {
            Open
        } else if c == '#' {
            Tree
        } else {
            panic!(format!("Impossible to convert {} to Square", c))
        }
    }
}

impl Add<&Steps> for Position {
    type Output = Self;

    fn add(self, steps: &Steps) -> Self {
        Self(
            self.0 + steps.0,
            self.1 + steps.1,
        )
    }
}
