pub mod area_map {
    use std::ops::Add;
    use std::str::FromStr;

    #[derive(Copy, Clone, Debug)]
    pub struct Position { x: usize, y: usize }

    impl Position {
        pub fn new(x: usize, y: usize) -> Position {
            Position { x, y }
        }
    }

    impl Add<Position> for Position {
        type Output = Self;

        fn add(self, p: Position) -> Self {
            Self {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    pub struct AreaMap<T: FromChar> {
        pub map: Vec<Vec<T>>,
        width: usize,
        height: usize,
        pub position: Position,
        has_pac_man_approach: bool,
    }

    impl<T: FromChar + Clone + Eq> AreaMap<T> {
        pub fn new(s: &str) -> Self {
            let map = s.lines().map(|line| line.chars().map(|c| T::from_char(c)).collect::<Vec<T>>()).collect::<Vec<Vec<T>>>();
            AreaMap {
                width: map[0].len(),
                height: map.len(),
                map,
                position: Position { x: 0, y: 0 },
                has_pac_man_approach: false,
            }
        }

        pub fn move_to(&mut self, p: Position) {
            self.position = p.clone();
        }

        pub fn move_by(&mut self, p: Position) {
            self.position = self.position + p;
        }

        pub fn is_on_bottom_map(&self) -> bool {
            if self.position.y == 0 {
                return false;
            }
            self.position.y.clone() - 1 == self.height.clone()
        }

        pub fn get_location(&self) -> T {
            if self.has_pac_man_approach {
                return self.map[self.position.y % self.height][self.position.x % self.width].clone();
            }
            self.map[self.position.y][self.position.x].clone()
        }

        pub fn is_in(&self, p: T) -> bool {
            self.get_location() == p
        }

        pub fn set_pac_man_approach(&mut self, b: bool) {
            self.has_pac_man_approach = b;
        }

        pub fn is_inside_base_map(&self) -> bool {
            self.position.x < self.width && self.position.y < self.height
        }
    }

    impl<T: FromChar + Clone + Eq> FromStr for AreaMap<T> {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(AreaMap::new(s))
        }
    }

    pub trait FromChar {
        fn from_char(c: char) -> Self;
    }
}