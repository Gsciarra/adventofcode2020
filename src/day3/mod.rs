use crate::day3::Square::{Open, Tree};
use crate::utils::area_map::area_map::{AreaMap, FromChar, Position};
use crate::utils::{get_input, print_solution};

fn count_trees_on_path(area_map: &mut AreaMap<Square>, position: &Position) -> usize {
    area_map.move_to(Position::new(0, 0));
    let mut total_trees = 0;

    while !area_map.is_on_bottom_map() {
        if area_map.is_in(Square::Tree) {
            total_trees = total_trees + 1;
        }

        area_map.move_by(position.clone());
    }

    total_trees
}

pub fn solution1() {
    let mut area_map = get_input::<AreaMap<Square>>("input-day3.txt").unwrap();
    area_map.set_pac_man_approach(true);
    print_solution(
        3,
        1,
        count_trees_on_path(&mut area_map, &Position::new(3, 1)),
    );
}

pub fn solution2() {
    let mut area_map = get_input::<AreaMap<Square>>("input-day3.txt").unwrap();
    area_map.set_pac_man_approach(true);
    let paths: Vec<Position> = vec![
        Position::new(1, 1),
        Position::new(3, 1),
        Position::new(5, 1),
        Position::new(7, 1),
        Position::new(1, 2),
    ];
    print_solution(
        3,
        2,
        paths.iter().fold(1, |acc, position| {
            acc * count_trees_on_path(&mut area_map, position)
        }),
    );
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Square {
    Open,
    Tree,
}

impl FromChar for Square {
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
