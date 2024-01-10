use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };

    let lines = read_lines(filename);
    let min_location = min_location(&lines);
    println!("{min_location}");
}

fn min_location(lines: &[String]) -> u64 {
    let maps: Maps = Maps::from(&lines[2..]);
    let seeds = get_seeds(&lines[0]);
    let locations = get_locations(seeds, maps);
    *locations.iter().min().unwrap()
}

struct Range {
    src_begin: u64,
    dest_begin: u64,
    length: u64,
}

struct Maps {
    maps: Vec<Vec<Range>>,
    cur_line: usize,
}

impl Maps {
    fn new() -> Self {
        Self {
            maps: Vec::new(),
            cur_line: 0,
        }
    }

    fn from(lines: &[String]) -> Self {
        let mut maps = Maps::new();
        loop {
            if maps.cur_line >= lines.len() {
                break;
            }
            let next_map = maps.next_map(lines);
            maps.maps.push(next_map);
        }
        maps
    }

    fn next_map(&mut self, lines: &[String]) -> Vec<Range> {
        self.cur_line += 1;
        let mut cur_map: Vec<Range> = Vec::new();
        while self.cur_line < lines.len() && lines[self.cur_line].trim().len() > 0 {
            let numbers = get_numbers(&lines[self.cur_line]);
            cur_map.push(Range {
                src_begin: numbers[1],
                dest_begin: numbers[0],
                length: numbers[2],
            });
            self.cur_line += 1;
        }
        self.cur_line += 1;
        cur_map
    }
}

fn get_locations(seeds: Vec<u64>, maps: Maps) -> Vec<u64> {
    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        locations.push(get_location(seed, &maps));
    }
    locations
}

fn get_location(seed: u64, maps: &Maps) -> u64 {
    let mut next_mapping = seed;
    for map in &maps.maps {
        next_mapping = map_to_next(next_mapping, &map);
    }
    next_mapping
}

fn map_to_next(number: u64, map: &Vec<Range>) -> u64 {
    for range in map {
        if number >= range.src_begin && number < range.src_begin + range.length {
            let range_offset = number - range.src_begin;
            return range.dest_begin + range_offset;
        }
    }
    number
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}

fn get_numbers(nums_str: &str) -> Vec<u64> {
    nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_seeds(seeds_str: &str) -> Vec<u64> {
    let parts: Vec<&str> = seeds_str.split(':').collect();
    get_numbers(parts[1])
}
