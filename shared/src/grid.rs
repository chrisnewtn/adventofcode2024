use core::fmt;
use std::{cmp::PartialEq, collections::HashSet, str::FromStr};

use crate::{coord::Coord, direction::{Direction, DIRECTIONS}};

#[derive(Debug)]
pub struct Grid<T: fmt::Display> {
    pub tiles: Vec<T>,
    row_len: usize,
    col_len: usize,
}

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

impl<T: fmt::Display + PartialEq> Grid<T> {
    fn build_area(&self, coord: &Coord, tile: &T, area: &mut HashSet<Coord>) {
        for neighbor in self.get_neighbor_coords(coord) {
            if let Some(nt) = self.get_tile_by_coord(&neighbor) {
                if nt == tile && !area.contains(&neighbor) {
                    area.insert(neighbor.clone());
                    self.build_area(&neighbor, &tile, area);
                }
            }
        }
    }

    pub fn get_area_from_coord(&self, coord: &Coord) -> HashSet<Coord> {
        let tile = self.get_tile_by_coord(coord);

        if tile.is_none() {
            return HashSet::new();
        }

        let tile = tile.unwrap();

        let mut area: HashSet<Coord> = HashSet::from([coord.clone()]);

        self.build_area(coord, &tile, &mut area);

        area
    }

    pub fn get_all_coords_of_tile(&self, tile: &T) -> HashSet<Coord> {
        self.tiles.iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if t == tile {
                    self.index_to_coord(i)
                } else {
                    None
                }
            })
            .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq)]
    struct Tile(char);

    #[derive(Debug)]
    pub struct ParseTileError;

    impl FromStr for Tile {
        type Err = ParseTileError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.chars().last().unwrap()))
        }
    }

    impl fmt::Display for Tile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn maintains_its_string_representation() {
        let grid: Grid<Tile> = Grid::from_str("AAAA\nBBCD\nBBCC\nEEEC").unwrap();

        assert_eq!(
            grid.to_string(),
            "AAAA\nBBCD\nBBCC\nEEEC\n"
        );
    }

    #[test]
    fn can_find_a_coords_neighbours_of_the_same_tile() {
        let grid: Grid<Tile> = Grid::from_str("AAAA\nBBCD\nBBCC\nEEEC").unwrap();

        let tile_a = Coord { x: 0, y: 0 };

        assert_eq!(
            grid.get_area_from_coord(&tile_a),
            HashSet::from([
                tile_a,
                Coord { x: 1, y: 0 },
                Coord { x: 2, y: 0 },
                Coord { x: 3, y: 0 },
            ])
        );

        let tile_b = Coord { x: 0, y: 1 };

        assert_eq!(
            grid.get_area_from_coord(&tile_b),
            HashSet::from([
                tile_b,
                Coord { x: 1, y: 1 },
                Coord { x: 0, y: 2 },
                Coord { x: 1, y: 2 },
            ])
        );

        let tile_c = Coord { x: 2, y: 1 };

        assert_eq!(
            grid.get_area_from_coord(&tile_c),
            HashSet::from([
                tile_c,
                Coord { x: 2, y: 2 },
                Coord { x: 3, y: 2 },
                Coord { x: 3, y: 3 },
            ])
        );

        let tile_d = Coord { x: 3, y: 1 };

        assert_eq!(
            grid.get_area_from_coord(&tile_d),
            HashSet::from([
                tile_d,
            ])
        );
    }
}
