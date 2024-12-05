use std::{fs::File, io::{self, BufRead}, path::Path, slice::Iter};

#[derive(Debug)]
struct Cell {
    content: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cells {
    all: Vec<Cell>,
    row_len: usize,
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

    pub fn get_neighbor<'a>(&self, direction: &Direction, cells: &'a Cells) -> Option<&'a Cell> {
        if let Some(coord) = self.neighbor_coord(&direction) {
            let cell_index = coord.y * cells.row_len + coord.x;

            if cell_index >= cells.all.len() {
                return None;
            }

            let cell = &cells.all[cell_index];

            if cell.x == coord.x && cell.y == coord.y {
                return Some(cell);
            }
        }
        None
    }

    pub fn has_word<'a>(&self, word: &str, direction: &Direction, cells: &'a Cells) -> bool {
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

    pub fn match_count(&self, word: &str, cells: &Cells) -> usize {
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

    for cell in cells.all.iter() {
        if cell.content != 'X' {
            continue;
        }

        // println!("cell: {:?}", cell);

        let count = cell.match_count("XMAS", &cells);

        // println!("matches: {}", count);

        total_matches += count;
    }

    println!("total matches: {}", total_matches); // answer: 2370
}

fn get_cells() -> Cells {
    let mut cells: Vec<Cell> = Vec::new();
    let mut line_len: usize = 0;

    if let Ok(lines) = read_lines("./input") {
        for (y, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                for (x, content) in line.chars().enumerate() {
                    cells.push(Cell {x, y, content});
                }
                if line_len == 0 {
                    line_len = line.len();
                }
            }
        }
    }

    Cells {
        all: cells,
        row_len: line_len,
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
