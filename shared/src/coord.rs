use core::fmt;
use std::{cmp::{Ordering, PartialEq}, ops::{Add, Sub}};

use crate::direction::Direction;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{},y:{})", self.x, self.y)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<Direction> for Coord {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => self - Self { y: 1, x: 0 },
            Direction::East => self + Self { y: 0, x: 1},
            Direction::South => self + Self { y: 1, x: 0},
            Direction::West => self - Self { y: 0, x: 1 },
        }
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
