use core::fmt;
use std::{ops::{Add, Sub}, str::FromStr};

#[derive(Debug)]
pub struct Grid<T: fmt::Display> {
    pub tiles: Vec<T>,
    row_len: usize,
    col_len: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
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

impl<T: fmt::Display> Grid<T> {
    fn validate_coord(&self, coord: &Coord) -> bool {
        coord.x < self.col_len && coord.y < self.row_len
    }

    fn coord_to_index(&self, coord: &Coord) -> Option<usize> {
        if self.validate_coord(coord) {
            Some(self.row_len * coord.y + coord.x)
        } else {
            None
        }
    }

    pub fn index_to_coord(&self, index: usize) -> Option<Coord> {
        if self.tiles.get(index).is_none() {
            return None;
        }
        Some(Coord {
            y: index / self.col_len,
            x: index % self.col_len,
        })
    }

    pub fn get_tile_by_coord(&self, coord: &Coord) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            Some(&self.tiles[index])
        } else {
            None
        }
    }

    pub fn get_neighbor_coord(&self, coord: &Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::North if coord.y == 0 => None,
            Direction::East if coord.x == self.col_len - 1 => None,
            Direction::South if coord.y == self.row_len - 1 => None,
            Direction::West if coord.x == 0 => None,
            _ => Some(coord.clone() + direction.clone()),
        }
    }

    pub fn get_neighbor_coords(&self, coord: &Coord) -> Vec<Coord> {
        DIRECTIONS.iter().filter_map(|d| self.get_neighbor_coord(coord, d)).collect()
    }

    pub fn get_neighbor_tile(&self, coord: &Coord, direction: Direction) -> Option<&T> {
        if let Some(nc) = self.get_neighbor_coord(coord, &direction) {
            self.get_tile_by_coord(&nc)
        } else {
            None
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for row in 0..self.row_len {
            for col in 0..self.col_len {
                s = format!("{}{}", s, self.tiles[(row * self.row_len) + col]);
            }
            s = format!("{}\n", s);
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct ParseGridError;

impl<T: FromStr + fmt::Display> FromStr for Grid<T> {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_len = s.lines().count();
        let col_len = s.lines().next().unwrap().len();
        let mut tiles = Vec::new();

        for line in s.lines() {
            for c in line.chars() {
                if let Ok(tile) = T::from_str(&c.to_string()) {
                    tiles.push(tile);
                }
            }
        }

        Ok(Self {
            tiles,
            row_len,
            col_len,
        })
    }
}
