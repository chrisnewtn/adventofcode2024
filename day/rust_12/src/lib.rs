use core::fmt;
use std::{collections::HashSet, str::FromStr};
use shared::{coord::Coord, grid::Grid};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Plant(char);

impl fmt::Display for Plant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ParsePlantError;

impl FromStr for Plant {
    type Err = ParsePlantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().last().unwrap()))
    }
}

pub fn get_kinds_of_plant(grid: &Grid<Plant>) -> HashSet<Plant> {
    let plants: HashSet<Plant> = grid.tiles.iter().map(|p| p.clone()).collect();
    plants
}

pub fn get_areas_of_plant(grid: &Grid<Plant>, plant: &Plant) -> Vec<HashSet<Coord>> {
    let mut tiles_of_plant = grid.get_all_coords_of_tile(plant);
    let mut areas: Vec<HashSet<Coord>> = Vec::new();

    while let Some(coord) = tiles_of_plant.iter().next() {
        let area = grid.get_area_from_coord(coord);

        tiles_of_plant = tiles_of_plant.difference(&area).map(|c| c.clone()).collect();

        areas.push(area);
    }

    areas
}

pub fn get_perimeter_of_coord(grid: &Grid<Plant>, coord: &Coord) -> usize {
    let tile = grid.get_tile_by_coord(coord).unwrap();
    let neighbors = grid.get_neighbor_coords(coord);

    neighbors.iter().fold(4, |total, n| {
        if let Some(n_tile) = grid.get_tile_by_coord(n) {
            if n_tile == tile {
                return total - 1;
            }
        }
        total
    })
}

pub fn get_perimeter_of_area(grid: &Grid<Plant>, area: &HashSet<Coord>) -> usize {
    area.iter().fold(0, |t, c| t + get_perimeter_of_coord(grid, c))
}

pub fn price_area(grid: &Grid<Plant>, area: &HashSet<Coord>) -> usize {
    area.len() * get_perimeter_of_area(grid, area)
}

pub fn price_grid(grid: &str) -> usize {
    let grid: Grid<Plant> = Grid::from_str(grid).unwrap();
    let mut price = 0;

    for plant_kind in get_kinds_of_plant(&grid) {
        for area in get_areas_of_plant(&grid, &plant_kind) {
            price += price_area(&grid, &area);
        }
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    fn large_example() -> String {
    "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE".trim().to_string()
    }

    #[test]
    fn maintains_its_string_representation() {
        let grid: Grid<Plant> = Grid::from_str("AAAA\nBBCD\nBBCC\nEEEC").unwrap();

        assert_eq!(
            grid.to_string(),
            "AAAA\nBBCD\nBBCC\nEEEC\n"
        );
    }

    #[test]
    fn can_get_the_kinds_of_plants_in_the_field() {
        let grid: Grid<Plant> = Grid::from_str("AAAA\nBBCD\nBBCC\nEEEC").unwrap();

        assert_eq!(
            get_kinds_of_plant(&grid),
            HashSet::from([Plant('A'), Plant('B'), Plant('C'), Plant('D'), Plant('E')])
        );
    }

    #[test]
    fn can_get_the_areas_occupied_by_a_kind_of_plant() {
        let grid: Grid<Plant> = Grid::from_str("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO").unwrap();

        let left = get_areas_of_plant(&grid, &Plant('X'));

        let right = vec![
            HashSet::from([Coord { x: 1, y: 1 }]),
            HashSet::from([Coord { x: 3, y: 1 }]),
            HashSet::from([Coord { x: 1, y: 3 }]),
            HashSet::from([Coord { x: 3, y: 3 }])
        ];

        assert_eq!(left.len(), right.len());

        for area in left {
            assert!(right.contains(&area));
        }
    }

    #[test]
    fn includes_the_edge_of_the_map_in_the_perimeter() {
        let grid: Grid<Plant> = Grid::from_str(&large_example()).unwrap();

        assert_eq!(
            get_perimeter_of_coord(&grid, &Coord { x: 0, y: 0 }),
            2
        );
    }

    #[test]
    fn can_get_the_perimeter_of_an_area() {
        let grid: Grid<Plant> = Grid::from_str("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO").unwrap();

        let area = HashSet::from([Coord { x: 1, y: 1 }]);

        assert_eq!(
            get_perimeter_of_area(&grid, &area),
            4
        );
    }

    #[test]
    fn each_part_of_the_large_example_as_expected() {
        let grid: Grid<Plant> = Grid::from_str(&large_example()).unwrap();

        let region_r_0 = grid.get_area_from_coord(&Coord { x: 0, y: 0 });

        assert_eq!(region_r_0.len(), 12,
            "Region R has incorrect area.");

        assert_eq!(get_perimeter_of_area(&grid, &region_r_0), 18,
            "Region R has incorrect perimeter.");

        assert_eq!(
            price_area(&grid, &region_r_0),
            216
        );
    }

    #[test]
    fn prices_the_large_example_grid_as_1930() {
        assert_eq!(
            price_grid(&large_example()),
            1930
        );
    }
}
