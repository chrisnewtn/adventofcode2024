use std::{fmt, ops::Range, str::FromStr};

#[derive(Debug)]
pub enum DiskBlock {
    File { id: usize },
    FreeSpace,
}

impl fmt::Display for DiskBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let DiskBlock::File { id } = self {
            write!(f, "{}", id)
        } else {
            write!(f, ".")
        }
    }
}

pub struct Disk {
    pub map: Vec<DiskBlock>,
}

impl Disk {
    pub fn block_string(&self) -> String {
        self.map.iter().map(|b| b.to_string()).collect()
    }

    fn first_free_index(&self, start: usize) -> Option<usize> {
        let map = &self.map[start..];

        for (index, block) in map.iter().enumerate() {
            if let DiskBlock::FreeSpace = block {
                return Some(start + index);
            }
        }
        None
    }

    fn last_occupied_index(&self) -> Option<usize> {
        for (index, block) in self.map.iter().enumerate().rev() {
            if let DiskBlock::File { id: _ } = block {
                return Some(index);
            }
        }
        None
    }

    pub fn compress(&mut self) {
        let mut last_free_index = 0;

        while let Some(free_index) = self.first_free_index(last_free_index) {
            last_free_index = free_index;

            if let Some(occupied_index) = self.last_occupied_index() {
                if free_index > occupied_index {
                    break;
                }
                self.map.swap(free_index, occupied_index);
            } else {
                break;
            }
        }
    }

    fn get_free_block_range(&self, size: usize, end_index: usize) -> Option<Range<usize>> {
        let mut this_range: Option<Range<usize>> = None;

        for (i, block) in self.map[0..end_index].iter().enumerate() {
            if let DiskBlock::FreeSpace = block {
                if let Some(old_range) = this_range {
                    let new_range = old_range.start..i + 1;

                    if new_range.len() == size {
                        return Some(new_range);
                    } else {
                        this_range = Some(new_range);
                    }
                } else {
                    this_range = Some(i..i + 1);

                    if size == 1 {
                        return this_range;
                    }
                }
            } else if this_range.is_some() {
                this_range = None;
            }
        }

        None
    }

    fn occupied_block_ranges(&self) -> Vec<Range<usize>> {
        let mut occupied_block_ranges = Vec::new();

        let mut this_id: Option<&usize> = None;
        let mut this_range: Option<Range<usize>> = None;

        for (i, block) in self.map.iter().enumerate() {
            if let DiskBlock::File { id } = block {
                if this_range.is_none() {
                    this_id = Some(id);
                    this_range = Some(i..i + 1);
                } else if let Some(seen_id) = this_id {
                    if seen_id == id {
                        let old_range = this_range.unwrap();
                        this_range = Some(old_range.start..old_range.end + 1);
                    } else {
                        occupied_block_ranges.push(this_range.unwrap());
                        this_id = Some(id);
                        this_range = Some(i..i + 1);
                    }
                }
            } else if let Some(range) = this_range {
                occupied_block_ranges.push(range);
                this_id = None;
                this_range = None;
            }
        }

        if let Some(range) = this_range {
            occupied_block_ranges.push(range);
        }

        occupied_block_ranges
    }

    pub fn compress_part_two(&mut self) {
        let mut occupied_block_ranges = self.occupied_block_ranges();

        occupied_block_ranges.reverse();

        for range in occupied_block_ranges {
            if let Some(free_range) = self.get_free_block_range(range.len(), range.start) {
                for (i, free_i) in free_range.enumerate() {
                    self.map.swap(free_i, range.start + i);
                }
            }
        }

    }

    pub fn checksum(&self) -> u64 {
        let mut total: u64 = 0;

        for (i, block) in self.map.iter().enumerate() {
            if let DiskBlock::File { id } = block {
                let id_digit = u32::try_from(id.clone()).unwrap();
                let position = u32::try_from(i).unwrap();

                total = total + u64::try_from(position * id_digit).unwrap();
            }
        }

        total
    }
}

#[derive(Debug)]
pub struct ParseDiskError;

impl FromStr for Disk {
    type Err = ParseDiskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let mut map: Vec<DiskBlock> = Vec::new();

        for (id, chunk) in chars.chunks(2).enumerate() {
            let blocks = chunk[0].to_digit(10).unwrap();

            for _ in 0..blocks {
                map.push(DiskBlock::File { id });
            }

            if let Some(e) = chunk.get(1) {
                let empties = e.to_digit(10).unwrap();

                for _ in 0..empties {
                    map.push(DiskBlock::FreeSpace);
                }
            }
        }

        Ok(Self {
            map,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_into_a_block_string() {
        assert_eq!(
            Disk::from_str("2333133121414131402").unwrap().block_string(),
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );
    }

    #[test]
    fn compresses() {
        let mut disk = Disk::from_str("2333133121414131402").unwrap();

        disk.compress();

        assert_eq!(
            disk.block_string(),
            "0099811188827773336446555566..............".to_string()
        );
    }

    #[test]
    fn compresses_to_satisfy_part_two() {
        let mut disk = Disk::from_str("2333133121414131402").unwrap();

        disk.compress_part_two();

        assert_eq!(
            disk.block_string(),
            "00992111777.44.333....5555.6666.....8888..".to_string()
        );
    }

    #[test]
    fn can_calculate_a_checksum() {
        let mut disk = Disk::from_str("2333133121414131402").unwrap();

        disk.compress();

        assert_eq!(
            disk.checksum(),
            1928
        );

        let mut disk = Disk::from_str("2333133121414131402").unwrap();

        disk.compress_part_two();

        assert_eq!(
            disk.checksum(),
            2858
        );
    }
}
