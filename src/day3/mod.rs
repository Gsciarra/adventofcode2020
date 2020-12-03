use std::str::FromStr;
use crate::day3::Square::{Open, Tree};
use std::ops::Add;
use crate::utils::get_input;

fn count_trees_on_path(location_map: &LocationMap, steps: &Steps) -> usize {
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

pub fn solution() {
    let location_map = get_input::<LocationMap>("input-day3.txt").unwrap();
    let paths: Vec<Steps> = vec![Steps(1, 1), Steps(3, 1), Steps(5, 1), Steps(7, 1), Steps(1, 2)];

    println!("Day 3 - part 1: {:?}", count_trees_on_path(&location_map, &Steps(3, 1)));
    println!("Day 3 - part 2: {:?}", paths.iter().fold(1, |acc, steps| acc * count_trees_on_path(&location_map, steps)));
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
