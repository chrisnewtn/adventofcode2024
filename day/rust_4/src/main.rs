use std::{fs::File, io::{self, BufRead}, path::Path, slice::Iter};

#[derive(Debug)]
struct Cell {
    content: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft
        ];
        DIRECTIONS.iter()
    }
}

impl Cell {
    pub fn neighbor_coord(&self, direction: &Direction) -> Option<Coord> {
        // println!("{:?} {:?}", self, direction);
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    return None;
                }
                Some(Coord { x: self.x, y: self.y - 1 })
            },
            Direction::UpRight => {
                if self.y == 0 {
                    return None;
                }
                Some(Coord { x: self.x + 1, y: self.y - 1 })
            },
            Direction::Right => {
                Some(Coord { x: self.x + 1, y: self.y })
            },
            Direction::DownRight => {
                Some(Coord { x: self.x + 1, y: self.y + 1 })
            },
            Direction::Down => {
                Some(Coord { x: self.x, y: self.y + 1 })
            },
            Direction::DownLeft => {
                if self.x == 0 {
                    return None;
                }
                Some(Coord { x: self.x - 1, y: self.y + 1 })
            },
            Direction::Left => {
                if self.x == 0 {
                    return None;
                }
                Some(Coord { x: self.x - 1, y: self.y })
            },
            Direction::UpLeft => {
                if self.x == 0 || self.y == 0 {
                    return None;
                }
                Some(Coord { x: self.x - 1, y: self.y - 1 })
            }
        }
    }

    pub fn get_neighbor<'a>(&self, direction: &Direction, cells: &'a Vec<Cell>) -> Option<&'a Cell> {
        if let Some(coord) = self.neighbor_coord(&direction) {
            for cell in cells.into_iter() {
                if cell.x == coord.x && cell.y == coord.y {
                    return Some(cell);
                }
            }
        }
        None
    }

    pub fn has_word<'a>(&self, word: &str, direction: &Direction, cells: &'a Vec<Cell>) -> bool {
        if let Some(first_char) = word.chars().nth(0) {
            if first_char != self.content {
                return false;
            }
        }

        let mut word_cells: Vec<&Cell> = vec![&self];

        for _ in word[1..].chars() {
            if let Some(neighbor) = word_cells.last().unwrap().get_neighbor(&direction, &cells) {
                word_cells.push(neighbor);
            } else {
                break;
            }
        }

        word_from_cells(word_cells) == word
    }

    pub fn match_count(&self, word: &str, cells: &Vec<Cell>) -> usize {
        let mut count: usize = 0;

        for direction in Direction::iterator() {
            if self.has_word(&word, direction, &cells) {
                count += 1;
            }
        }

        count
    }
}

fn word_from_cells(cells: Vec<&Cell>) -> String {
    let mut word = String::new();

    for cell in cells {
        word.push(cell.content);
    }

    word
}

fn main() {
    let cells = get_cells();
    let mut total_matches = 0;

    for cell in cells.iter() {
        if cell.content != 'X' {
            continue;
        }

        // println!("cell: {:?}", cell);

        let count = cell.match_count("XMAS", &cells);

        // println!("matches: {}", count);

        total_matches += count;
    }

    println!("total matches: {}", total_matches);
}

fn get_cells() -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for (y, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                for (x, content) in line.chars().enumerate() {
                    cells.push(Cell {x, y, content});
                }
            }
        }
    }

    cells
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
