use core::fmt;
use std::{cmp, str::FromStr, sync::{Arc, Mutex}, thread, time::Instant};

pub struct Stone {
    number: usize,
}

impl Stone {
    pub fn blink(&self) -> (Self, Option<Self>) {
        if self.number == 0 {
            return (
                Self {
                    number: 1,
                },
                None
            )
        }

        let number_as_string = self.number.to_string();

        if number_as_string.chars().count() % 2 == 0 {
            let (left, right) = number_as_string.split_at(number_as_string.len() / 2);

            (
                Self {
                    number: left.parse().unwrap(),
                },
                Some(Self {
                    number: right.parse().unwrap(),
                })
            )
        } else {
            (
                Self {
                    number: self.number * 2024,
                },
                None
            )
        }
    }
}

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

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

pub struct StoneLine {
    pub stones: Vec<usize>,
    pub blinks: usize,
}

const STONES_PER_THREAD: usize = 5000000;

fn get_number_of_threads(stones: &usize) -> usize {
    cmp::min((stones / STONES_PER_THREAD) + 1, 4)
}

impl StoneLine {
    pub fn blink_all(&mut self) {
        let num_threads = get_number_of_threads(&self.stones.len());

        println!("num_threads: {}", num_threads);

        // if num_threads == 1 {
        //     self.stones = blink_stones(&mut self.stones);
        //     return;
        // }

        let new_line = Arc::new(Mutex::new(Vec::<usize>::new()));

        let stones: Vec<usize> = self.stones.drain(..).collect();

        let handles = stones.chunks(num_threads).map(|chunk| {
            let nl = Arc::clone(&new_line);
            // let mut to_send = Vec::from(chunk);
            let to_send = Arc::new(chunk);

            thread::spawn(move || {
                let mut new_line_chunk = blink_stones(&to_send);

                let mut nl = nl.lock().unwrap();

                nl.append(&mut new_line_chunk);
            })
        });

        for handle in handles {
            handle.join().unwrap();
        }

        self.stones = (*new_line.lock().unwrap().clone()).to_vec();

        // let mut inserts: Vec<(usize, usize)> = Vec::new();
        //
        // for i in 0..self.stones.len() {
        //     let (left, right) = blink(&self.stones[i]);
        //
        //     self.stones[i] = left;
        //
        //     if let Some(right) = right {
        //         inserts.push((i + 1, right));
        //     }
        // }
        //
        // for insert_i in 0..inserts.len() {
        //     let (stone_i, stone) = inserts[insert_i];
        //     self.stones.insert(stone_i + insert_i, stone);
        // }

//         let mut new_line: Vec<usize> = Vec::with_capacity(self.stones.capacity());
//
//         for stone in self.stones.iter() {
//             let (left, right) = blink(*stone);
//
//             new_line.push(left);
//
//             if let Some(right) = right {
//                 new_line.push(right);
//             }
//         }
//
//         self.stones = new_line;

        // self.stones = self.stones.iter().flat_map(|stone| {
        //         let (left, right) = blink(stone);
        //
        //         if let Some(right) = right {
        //             vec![left, right]
        //         } else {
        //             vec![left]
        //         }
        //     })
        //     .collect()

        // let mut new_line: VecDeque<usize> = VecDeque::with_capacity(self.stones.capacity());
        //
        // while let Some(stone) = self.stones.pop_front() {
        //     let (left, right) = blink(stone);
        //
        //     new_line.push_back(left);
        //
        //     if let Some(right) = right {
        //         new_line.push_back(right);
        //     }
        // }
        //
        // self.stones = new_line;
    }

    pub fn blink_all_times(&mut self, times: usize) {
        for _ in 0..times {
            let now = Instant::now();
            self.blink_all();
            self.blinks = self.blinks + 1;
            let elapsed = now.elapsed();
            println!("{} stones after {} blink(s) in {:.2?}", self.stones.len(), self.blinks, elapsed);
        }
    }
}

impl fmt::Display for StoneLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: String = self.stones.iter()
            .map(|s| format!("{} ", s))
            .collect();

        write!(f, "{}", string.trim())
    }
}

#[derive(Debug)]
pub struct ParseStoneLineError;

impl FromStr for StoneLine {
    type Err = ParseStoneLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            stones,
            blinks: 0
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
            stone_line.stones.len(),
            55312
        );
    }
}
