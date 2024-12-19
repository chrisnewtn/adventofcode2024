use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::{self}, str::FromStr};

mod grid;

pub use grid::{Grid, Coord, DIRECTIONS};

#[derive(Debug, Eq)]
#[repr(u8)]
pub enum Trail {
    Start = 0,
    Path(u8),
    End = 9,
}

impl Trail {
    pub fn elevation(&self) -> u8 {
        match self {
            Trail::Start => 0,
            Trail::Path(n) => n.clone(),
            Trail::End => 9,
        }
    }

    pub fn gradient(&self, other: &Trail) -> u8 {
        other.elevation() - self.elevation()
    }
}

impl fmt::Display for Trail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Trail::Start => write!(f, "0"),
            Trail::Path(n) => write!(f, "{}", n),
            Trail::End => write!(f, "9"),
        }
    }
}

impl PartialOrd for Trail {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.elevation().cmp(&other.elevation())
    }
}

impl PartialEq for Trail {
    fn eq(&self, other: &Self) -> bool {
        self.elevation() == other.elevation()
    }
}

#[derive(Debug)]
pub struct ParseTrailError;

impl FromStr for Trail {
    type Err = ParseTrailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.parse::<u8>();

        if n.is_err() {
            return Err(ParseTrailError);
        }

        match n.unwrap() {
            0 => Ok(Trail::Start),
            9 => Ok(Trail::End),
            x => {
                if (1..=8).contains(&x) {
                    Ok(Trail::Path(x.clone()))
                } else {
                    Err(ParseTrailError)
                }
            }
        }
    }
}

impl Grid<Trail> {
    pub fn get_trail_head_coords(&self) -> Vec<Coord> {
        self.tiles.iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if *t == Trail::Start {
                    self.index_to_coord(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_trails(&self, coord: &Coord, trail_map: &mut HashMap<Coord, Vec<Coord>>) {
        let path_points = self.get_possible_path(coord);

        trail_map.insert(coord.clone(), path_points.clone());

        for next_coord in path_points {
            self.build_trails(&next_coord, trail_map);
        }
    }

    pub fn get_trails_from_coord(&self, coord: &Coord) -> HashMap<Coord, Vec<Coord>> {
        let mut trail_map = HashMap::new();

        self.build_trails(coord, &mut trail_map);

        trail_map
    }

    pub fn get_trail_score(&self, coord: &Coord) -> usize {
        let trail_map = self.get_trails_from_coord(coord);

        trail_map.iter()
            .filter(|(_, val)| val.is_empty())
            .fold(HashSet::new(), |mut ends, (trail_coord, _)| {
                if let Some(tile) = self.get_tile_by_coord(trail_coord) {
                    if tile == &Trail::End {
                        ends.insert(trail_coord);
                    }
                }
                ends
            })
            .len()
    }

    pub fn total_score(&self) -> usize {
        self.get_trail_head_coords().iter()
            .fold(0, |score, trail_head| score + self.get_trail_score(trail_head))
    }

    pub fn get_possible_path(&self, coord: &Coord) -> Vec<Coord> {
        let tile = self.get_tile_by_coord(coord);

        if tile.is_none() {
            return vec![];
        }

        let tile = tile.unwrap();

        if tile == &Trail::End {
            return vec![];
        }

        let elevation = tile.elevation();

        self.get_neighbor_coords(coord)
            .iter()
            .filter_map(|neighbor| {
                let neighbor_tile = self.get_tile_by_coord(neighbor).unwrap();
                let neighbor_elevation = neighbor_tile.elevation();

                if neighbor_elevation <= elevation {
                    None
                } else if neighbor_elevation - elevation == 1 {
                    Some(neighbor.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use grid::Direction;

    use super::*;

    fn fixture() -> String {
        "89010123
         78121874
         87430965
         96549874
         45678903
         32019012
         01329801
         10456732".lines().map(|l| format!("{}\n", l.trim())).collect()
    }

    #[test]
    fn maintains_its_string_representation() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.to_string(),
            fixture()
        );
    }

    #[test]
    fn can_retrieve_tiles_by_coord() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.get_tile_by_coord(&Coord {x: 0, y: 0}),
            Some(&Trail::Path(8))
        );

        assert_eq!(
            grid.get_tile_by_coord(&Coord {x: 1, y: 0}),
            Some(&Trail::End)
        );

        assert_eq!(
            grid.get_tile_by_coord(&Coord {x: 2, y: 0}),
            Some(&Trail::Start)
        );

        assert_eq!(
            grid.get_tile_by_coord(&Coord {x: 0, y: 7}),
            Some(&Trail::Path(1))
        );
    }

    #[test]
    fn can_access_neighbouring_coords() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        let coord = Coord {x: 4, y: 3};

        assert_eq!(
            grid.get_tile_by_coord(&coord),
            Some(&Trail::Path(9))
        );

        assert_eq!(
            grid.get_neighbor_tile(&coord, Direction::North),
            Some(&Trail::Start)
        );

        assert_eq!(
            grid.get_neighbor_tile(&coord, Direction::East),
            Some(&Trail::Path(8))
        );

        assert_eq!(
            grid.get_neighbor_tile(&coord, Direction::South),
            Some(&Trail::Path(8))
        );

        assert_eq!(
            grid.get_neighbor_tile(&coord, Direction::West),
            Some(&Trail::Path(4))
        );
    }

    #[test]
    fn can_list_all_trail_heads() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.get_trail_head_coords(),
            vec![
                Coord {x: 2, y: 0},
                Coord {x: 4, y: 0},
                Coord {x: 4, y: 2},
                Coord {x: 6, y: 4},
                Coord {x: 2, y: 5},
                Coord {x: 5, y: 5},
                Coord {x: 0, y: 6},
                Coord {x: 6, y: 6},
                Coord {x: 1, y: 7},
            ]
        );
    }

    #[test]
    fn can_list_all_adjacent_passable_coords() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 6, y: 4}
            ),
            vec![Coord {x: 6, y: 5}],
            "Expected the coord to the immediate south from the start."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 6, y: 5}
            ),
            vec![Coord {x: 7, y: 5}],
            "Expected the coord to the east from the second tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 7, y: 5}
            ),
            vec![Coord {x: 7, y: 4}],
            "Expected the coord to the north of the third tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 7, y: 4}
            ),
            vec![Coord {x: 7, y: 3}],
            "Expected the coord to the north of the fourth tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 7, y: 3}
            ),
            vec![Coord {x: 7, y: 2}],
            "Expected the coord to the north of the fifth tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 7, y: 2}
            ),
            vec![Coord {x: 6, y: 2}],
            "Expected the coord to the west of the sixth tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 6, y: 2}
            ),
            vec![
                Coord {x: 6, y: 1}, //8a
                Coord {x: 6, y: 3}, //8b
            ],
            "Expected the coords to the north and south of the seventh tile."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 6, y: 1}
            ),
            vec![
                Coord {x: 5, y: 1}, //8a9
            ],
            "Expected the coord to the west of tile 8a."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 5, y: 1}
            ),
            vec![
                Coord {x: 5, y: 2}, //8a10
            ],
            "Expected the coord to the south of tile 8a9."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 5, y: 2}
            ),
            vec![],
            "Expected no coords after tile 8a10."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 6, y: 3}
            ),
            vec![
                Coord {x: 5, y: 3}, //8b9
            ],
            "Expected the coord to the west of tile 8b."
        );

        assert_eq!(
            grid.get_possible_path(
                &Coord {x: 5, y: 3}
            ),
            vec![
                Coord {x: 5, y: 2}, //8b10a
                Coord {x: 5, y: 4}, //8b10b
                Coord {x: 4, y: 3}, //8b10c
            ],
            "Expected the coords to the north, west and south of tile 8b9."
        );
    }

    #[test]
    fn can_find_the_score_of_a_trail_head() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.get_trail_score(&Coord {x: 6, y: 4}),
            3
        );
    }

    #[test]
    fn can_get_the_total_score_of_the_map() {
        let grid: Grid<Trail> = Grid::from_str(&fixture()).unwrap();

        assert_eq!(
            grid.total_score(),
            36
        );
    }
}
