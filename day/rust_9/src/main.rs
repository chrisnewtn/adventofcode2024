use std::str::FromStr;
use rust_9::Disk;

fn main() {
    let file = include_str!("../input");

    for line in file.lines() {
        let mut disk = Disk::from_str(&line).unwrap();

        disk.compress();
        println!("part 1 solution: {}", disk.checksum());

        let mut disk = Disk::from_str(&line).unwrap();

        disk.compress_part_two();
        println!("part 2 solution: {}", disk.checksum());
    }
}
