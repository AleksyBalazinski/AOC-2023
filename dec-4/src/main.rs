use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };

    let lines = read_lines(filename);
    let total_points = total_points(&lines);
    println!("total points: {total_points}");
}

fn total_points(lines: &[String]) -> i32 {
    let mut acc = 0;
    for line in lines {
        let game_numbers = game_numbers(line);
        acc += points_from_card(game_numbers);
    }
    acc
}

fn points_from_card(game_numbers: GameNumbers) -> i32 {
    let mut acc = 0;
    for winning_num in game_numbers.winning_numbers {
        if game_numbers.our_numbers.contains(&winning_num) {
            acc += 1;
        }
    }

    if acc == 0 {
        return 0;
    }

    1 << (acc - 1)
}

struct GameNumbers {
    winning_numbers: HashSet<i32>,
    our_numbers: HashSet<i32>,
}

fn game_numbers(line: &str) -> GameNumbers {
    let card = line.split(':').collect::<Vec<&str>>()[1];

    let numbers: Vec<&str> = card.split('|').collect();
    let winning_numbers = get_numbers(numbers[0]);
    let our_numbers = get_numbers(numbers[1]);

    GameNumbers {
        winning_numbers,
        our_numbers,
    }
}

fn get_numbers(nums_str: &str) -> HashSet<i32> {
    nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
