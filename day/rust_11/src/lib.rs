use core::fmt;
use std::{env, fs, io::{self, BufRead, Write}, path::PathBuf, str::FromStr, time::Instant};

use indicatif::ProgressBar;
use uuid::Uuid;

fn blink(number: usize) -> (usize, Option<usize>) {
    if number == 0 {
        return (1, None)
    }

    let number_as_string = number.to_string();

    if number_as_string.chars().count() % 2 == 0 {
        let (left, right) = number_as_string.split_at(number_as_string.len() / 2);

        (left.parse().unwrap(), Some(right.parse().unwrap()))
    } else {
        (number * 2024, None)
    }
}

fn split_blink(number: &str) -> Option<(&str, &str)> {
    if number.chars().count() % 2 == 0 {
        Some(number.split_at(number.len() / 2))
    } else {
        None
    }
}

fn blink_stones(stones: &[usize]) -> Vec<usize> {
    let mut new_line: Vec<usize> = Vec::with_capacity(stones.len());

    for stone in stones.iter() {
        let (left, right) = blink(*stone);

        new_line.push(left);

        if let Some(right) = right {
            new_line.push(right);
        }
    }

    new_line
}

fn blink_file(input_num_stones: usize, input: &PathBuf, output: &PathBuf) -> Result<usize, std::io::Error> {
    println!("processing stone dump: {:?}", input);

    let input_stones = fs::File::open(input)?;
    let reader = io::BufReader::new(input_stones);
    let pb = ProgressBar::new(input_num_stones.try_into().unwrap());

    let mut output_stones = fs::File::create(output)?;
    let mut num_input_stones = 0;
    let mut num_stones_total = 0;
    let mut num_stones_for_chunk = 0;
    let mut new_line = String::new();

    for line in reader.lines() {
        let line = line?;
        let stones = line.split_whitespace();

        for stone in stones {
            num_input_stones += 1;

            if let Some((left, right)) = split_blink(stone) {
                new_line.push_str(&format!(" {left} {right}"));
                num_stones_for_chunk += 2;
            } else {
                let number: usize = stone.parse().unwrap();

                if number == 0 {
                    new_line.push_str(" 1");
                } else {
                    new_line.push_str(&format!(" {}", number * 2024));
                }

                num_stones_for_chunk += 1;
            }

            if num_stones_for_chunk >= STONE_CHUNK_LENGTH {
                output_stones.write(format!("{}\n", new_line.trim_start()).as_bytes())?;
                new_line.clear();
                num_stones_total += num_stones_for_chunk;
                pb.inc(num_input_stones);
                num_stones_for_chunk = 0;
                num_input_stones = 0;
            }
        }
    }

    if num_stones_for_chunk > 0 {
        output_stones.write(format!("{}\n", new_line.trim()).as_bytes())?;
        new_line.clear();
        pb.inc(num_input_stones);
    }

    pb.finish_with_message(format!("processed {num_stones_total} stones to dump {:?}", output));

    Ok(num_stones_total)
}

fn gen_filename(id: &Uuid, blinks: &usize) -> PathBuf {
    let mut filename = gen_dirname(id);

    filename.push(blinks.to_string());

    filename
}

fn gen_dirname(id: &Uuid) -> PathBuf {
    let mut filename = PathBuf::new();

    filename.push(env::temp_dir());
    filename.push("adventofcode2024-day-11");
    filename.push(id.to_string());

    filename
}

const LOT_OF_STONES: usize = 10_000_000;
const STONE_CHUNK_LENGTH: usize = 100_000;

enum StoneStorage {
    Vector(Vec<usize>),
    File(PathBuf),
}

pub struct StoneLine {
    id: Uuid,
    stones: StoneStorage,
    pub blinks: usize,
    pub num_stones: usize,
}

impl StoneLine {
    pub fn blink_all(&mut self) {
        match &mut self.stones {
            StoneStorage::Vector(stones) => {
                let blinked_stones = blink_stones(&stones);

                self.num_stones = blinked_stones.len();
                self.stones = StoneStorage::Vector(blinked_stones);
                self.blinks = self.blinks + 1;
            }
            StoneStorage::File(old_filename) => {
                let new_filename = gen_filename(&self.id, &(self.blinks + 1));

                match blink_file(self.num_stones, old_filename, &new_filename) {
                    Ok(num_stones) => {
                        self.num_stones = num_stones;
                        self.stones = StoneStorage::File(new_filename);
                        self.blinks = self.blinks + 1;
                    }
                    Err(err) => {
                        panic!("{}", err);
                    }
                }
            }
        }
    }

    fn dump_stones_to_disk(&mut self) -> Result<PathBuf, std::io::Error> {
        let stones: &mut Vec<usize> = match &mut self.stones {
            StoneStorage::Vector(v) => v,
            StoneStorage::File(f) => return Ok(f.clone()),
        };

        fs::create_dir_all(gen_dirname(&self.id))?;

        let filename = gen_filename(&self.id, &self.blinks);

        println!("dumping {} stones to {:?}", stones.len(), filename);

        let mut file = fs::File::create(filename.clone())?;

        for chunk in stones.chunks(STONE_CHUNK_LENGTH) {
            let chunk = chunk.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            file.write(format!("{}\n", chunk.join(" ")).as_bytes())?;
        }

        self.stones = StoneStorage::File(filename.clone());

        Ok(filename)
    }

    pub fn blink_all_times(&mut self, times: usize) {
        for _ in 0..times {
            let now = Instant::now();
            self.blink_all();
            let elapsed = now.elapsed();

            println!("{} stones after {} blink(s) in {:.2?}", self.num_stones, self.blinks, elapsed);

            if self.num_stones > LOT_OF_STONES {
                if let Err(err) = self.dump_stones_to_disk() {
                    panic!("{}", err);
                }
            }
        }
    }
}

impl fmt::Display for StoneLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.stones {
            StoneStorage::Vector(stones) => {
                let string: String = stones.iter().map(|s| format!("{} ", s)).collect();
                write!(f, "{}", string.trim())
            }
            StoneStorage::File(path) => {
                write!(f, "stones stored at: {:?}", path)
            }
        }

    }
}

#[derive(Debug)]
pub struct ParseStoneLineError;

impl FromStr for StoneLine {
    type Err = ParseStoneLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones: Vec<usize> = s.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let num_stones = stones.len();

        Ok(Self {
            id: Uuid::new_v4(),
            stones: StoneStorage::Vector(stones),
            blinks: 0,
            num_stones,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maintains_its_string_representation() {
        let stone_line: StoneLine = "0 1 10 99 999".parse().unwrap();

        assert_eq!(
            stone_line.to_string(),
            "0 1 10 99 999"
        );
    }

    #[test]
    fn after_one_blink() {
        let mut stone_line: StoneLine = "0 1 10 99 999".parse().unwrap();

        stone_line.blink_all();

        assert_eq!(
            stone_line.to_string(),
            "1 2024 1 0 9 9 2021976"
        );
    }

    #[test]
    fn long_example() {
        let mut stone_line: StoneLine = "125 17".parse().unwrap();

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "253000 1 7"
        );

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "253 0 2024 14168"
        );

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "512072 1 20 24 28676032"
        );

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "512 72 2024 2 0 2 4 2867 6032"
        );

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
        );

        stone_line.blink_all();
        assert_eq!(
            stone_line.to_string(),
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }

    #[test]
    fn long_example_stone_count() {
        let mut stone_line: StoneLine = "125 17".parse().unwrap();

        stone_line.blink_all_times(25);

        assert_eq!(
            stone_line.num_stones,
            55312
        );
    }
}
