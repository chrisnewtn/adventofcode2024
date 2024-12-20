use core::fmt;
use std::{cell::RefCell, collections::HashSet, fs::File, io::{self, BufRead}, ops::Deref, path::Path, rc::Rc};
use uuid::Uuid;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn turn_right(direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TileKind {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTileKindError;

impl TileKind {
    pub fn from_char(c: &char) -> Result<Self, ParseTileKindError> {
        match c {
            '.' => Ok(TileKind::Empty),
            '#' => Ok(TileKind::Obstacle),
            '^' => Ok(TileKind::Guard(Direction::Up)),
            '>' => Ok(TileKind::Guard(Direction::Right)),
            'v' => Ok(TileKind::Guard(Direction::Down)),
            '<' => Ok(TileKind::Guard(Direction::Left)),
            _ => Err(ParseTileKindError)
        }
    }
    pub fn is_guard(&self) -> bool {
        match &self {
            TileKind::Guard(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
struct GridTile {
    pub id: Uuid,
    pub kind: RefCell<TileKind>,
    pub visited: RefCell<HashSet<Direction>>,
}

impl GridTile {
    pub fn new(kind: TileKind) -> Self {
        let visited = match &kind {
            TileKind::Guard(direction) => HashSet::from([direction.clone()]),
            _ => HashSet::new(),
        };

        Self {
            id: Uuid::new_v4(),
            kind: RefCell::new(kind),
            visited: RefCell::new(visited),
        }
    }

    pub fn set_visited(&self, direction: Direction) {
        self.visited.borrow_mut().insert(direction);
    }

    pub fn get_visited(&self) -> bool {
        !self.visited.borrow().is_empty()
    }

    pub fn turn_right(&self) {
        let kind = RefCell::clone(&self.kind);

        if let TileKind::Guard(direction) = kind.borrow().deref() {
            let new_direction = Direction::turn_right(&direction);
            self.set_visited(new_direction.clone());
            self.kind.replace(TileKind::Guard(new_direction));
        };
    }
}

impl fmt::Display for GridTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.kind.borrow() {
            TileKind::Empty => {
                let v = self.visited.borrow();

                let up_or_down = v.contains(&Direction::Up) || v.contains(&Direction::Down);
                let left_or_right = v.contains(&Direction::Left) || v.contains(&Direction::Right);

                if up_or_down && left_or_right {
                    write!(f, "+")
                } else if up_or_down {
                    write!(f, "|")
                } else if left_or_right {
                    write!(f, "-")
                } else {
                    write!(f, ".")
                }
            },
            TileKind::Obstacle => write!(f, "#"),
            TileKind::Guard(Direction::Up) => write!(f, "^"),
            TileKind::Guard(Direction::Right) => write!(f, ">"),
            TileKind::Guard(Direction::Down) => write!(f, "v"),
            TileKind::Guard(Direction::Left) => write!(f, "<"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    pub tiles: RefCell<Vec<Rc<GridTile>>>,
    pub row_len: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_write = String::new();

        for row in self.tiles.borrow().chunks(self.row_len) {
            for cell in row {
                to_write.push_str(&cell.to_string());
            }
            to_write.push('\n');
        }

        write!(f, "{}", to_write)
    }
}

enum MovementResult {
    Ok,
    TileIsNotGuard,
    NoNeighbor,
    Obstructed,
}

enum Neighbor<T> {
    Ok(T),
    NoNeighbor,
    Obstructed,
}

impl Grid {
    pub fn from_file(path: &str) -> Self {
        let mut tiles = Vec::new();
        let mut row_len = 0;

        if let Ok(lines) = read_lines(path) {
            for (_y, line_result) in lines.enumerate() {
                if let Ok(line) = line_result {
                    if row_len == 0 {
                        row_len = line.len();
                    }
                    for (_x, content) in line.chars().enumerate() {
                        let kind = TileKind::from_char(&content).unwrap();
                        tiles.push(Rc::new(GridTile::new(kind)))
                    }
                }
            }
        }

        Self {
            tiles: RefCell::new(tiles),
            row_len,
        }
    }

    pub fn get_guard(&self) -> Option<Rc<GridTile>> {
        for tile in self.tiles.borrow().iter() {
            if tile.kind.borrow().is_guard() {
                return Some(Rc::clone(tile));
            }
        }
        None
    }

    fn get_unobstructed_neighbor(&self, tile: &Rc<GridTile>, direction: &Direction) -> Neighbor<Rc<GridTile>> {
        let coord = self.get_tile_coord(&tile);

        let tile_index = match direction {
            Direction::Up => (coord.y - 1) * self.row_len + coord.x,
            Direction::Right => coord.y * self.row_len + coord.x + 1,
            Direction::Down => (coord.y + 1) * self.row_len + coord.x,
            Direction::Left => coord.y * self.row_len + coord.x - 1,
        };

        if tile_index > 0 && tile_index < self.tiles.borrow().len() {
            let neighbor = Rc::clone(&self.tiles.borrow()[tile_index]);

            if *neighbor.kind.borrow() == TileKind::Obstacle {
                Neighbor::Obstructed
            } else {
                Neighbor::Ok(neighbor)
            }
        } else {
            Neighbor::NoNeighbor
        }
    }

    fn get_tile_index(&self, tile: &Rc<GridTile>) -> usize {
        self.tiles.borrow().iter().position(|t| t.id == tile.id).unwrap()
    }

    fn get_tile_coord(&self, tile: &Rc<GridTile>) -> Coord {
        let index = self.get_tile_index(&tile);

        Coord {
            x: index % self.row_len,
            y: index / self.row_len
        }
    }

    pub fn move_tile(&self, tile: Rc<GridTile>) -> MovementResult {
        let binding = RefCell::clone(&tile.kind);
        let kind = binding.borrow();

        if let TileKind::Guard(direction) = kind.deref() {
            match self.get_unobstructed_neighbor(&tile, &direction) {
                Neighbor::Ok(neighbor) => {
                    Rc::clone(&neighbor).set_visited(direction.clone());
                    tile.kind.swap(&neighbor.kind);
                    MovementResult::Ok
                }
                Neighbor::NoNeighbor => MovementResult::NoNeighbor,
                Neighbor::Obstructed => MovementResult::Obstructed,
            }
        } else {
            MovementResult::TileIsNotGuard
        }
    }
}

fn main() {
    let grid = Grid::from_file("./input");

    loop {
        let guard = grid.get_guard().unwrap();

        match grid.move_tile(guard) {
            MovementResult::Obstructed => {
                let guard = grid.get_guard().unwrap();
                guard.turn_right();
            },
            MovementResult::NoNeighbor => break,
            _ => ()
        }
    }

    println!("{}\n", grid);

    let visited = grid.tiles.borrow().iter()
        .filter(|t| t.get_visited())
        .collect::<Vec<_>>().len();

    println!("visited locations: {}", visited); // answer 5404


    // println!("test: {:?}", grid.tiles.borrow().get((10 * 9) + 7).unwrap())
}
