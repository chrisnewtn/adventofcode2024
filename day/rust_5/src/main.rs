use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs::File, io::{self, BufRead}, path::Path};
use regex::Regex;

#[derive(Debug)]
struct PageRules {
    before: HashMap<usize, HashSet<usize>>,
}

impl PageRules {
    pub fn new() -> Self {
        Self {
            before: HashMap::new(),
        }
    }
}

fn parse_input() -> (PageRules, Vec<Vec<usize>>) {
    let mut rules = PageRules::new();
    let mut updates = Vec::new();

    let rule_matcher = Regex::new(r"(?<left>\d+)\|(?<right>\d+)").unwrap();
    let page_patcher = Regex::new(r"(\d+)").unwrap();

    if let Ok(lines) = read_lines("./input") {
        for (_line_no, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                if let Some(captures) = rule_matcher.captures(&line) {
                    let left: usize = captures["left"].parse().unwrap();
                    let right: usize = captures["right"].parse().unwrap();

                    if None == rules.before.get(&left) {
                        rules.before.insert(left, HashSet::new());
                    }

                    rules.before.get_mut(&left).unwrap().insert(right);
                } else if page_patcher.is_match(&line) {
                    let mut update: Vec<usize> = Vec::new();

                    for capture in page_patcher.captures_iter(&line) {
                        update.push(capture.get(0).unwrap().as_str().parse().unwrap());
                    }

                    updates.push(update);
                }
            }
        }
    }

    (rules, updates)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let (rules, mut updates) = parse_input();

    let mut already_ordered_middles = 0;
    let mut fixed_middles = 0;

    for update in updates.iter_mut() {
        let unsorted = update.clone();

        update.sort_by(|a, b| {
            if let Some(before) = rules.before.get(a) {
                if before.contains(b) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });

        let middle = update.get(update.len() / 2).unwrap();

        if unsorted.eq(update) {
            already_ordered_middles += middle;
        } else {
            fixed_middles += middle;
        }
    }

    println!("already ordered middles: {}", already_ordered_middles); // part 1 answer: 5964
    println!("          fixed middles: {}", fixed_middles); // part 2 answer: 4719
}
