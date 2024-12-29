#[derive(Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West
];
