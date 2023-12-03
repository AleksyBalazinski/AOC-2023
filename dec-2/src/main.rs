use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };

    let lines = read_lines(filename);

    let possible_games = possible_games(&lines);
    let sum_power_sets = sum_power_sets(&lines);
    println!("possible games: {possible_games}");
    println!("sum of power of sets: {sum_power_sets}");
}

fn sum_power_sets(lines: &[String]) -> i32 {
    let mut acc = 0;
    for line in lines {
        let game: Vec<&str> = line.split(":").collect();
        let draws: Vec<&str> = game[1].split(';').collect();
        acc += power_set(&draws);
    }

    acc
}

fn power_set(draws: &Vec<&str>) -> i32 {
    let cube_counts: Vec<CubeCounts> = draws.iter().map(|x| get_cubes(x)).collect();
    let max_red = cube_counts.iter().map(|x| x.red).max();
    let max_green = cube_counts.iter().map(|x| x.green).max();
    let max_blue = cube_counts.iter().map(|x| x.blue).max();

    max_red.unwrap() * max_blue.unwrap() * max_green.unwrap()
}

fn possible_games(lines: &[String]) -> i32 {
    let mut acc = 0;
    for line in lines {
        let game: Vec<&str> = line.split(":").collect();
        let game_number: i32 = game[0].split(' ').collect::<Vec<&str>>()[1]
            .trim()
            .parse()
            .expect("Not a valid game number");
        let draws: Vec<&str> = game[1].split(';').collect();
        if is_game_possible(&draws) {
            acc += game_number;
        }
    }
    acc
}

fn is_game_possible(draws: &Vec<&str>) -> bool {
    for draw in draws {
        let cube_counts = get_cubes(draw);
        if cube_counts.red > 12 || cube_counts.green > 13 || cube_counts.blue > 14 {
            return false;
        }
    }

    true
}

struct CubeCounts {
    red: i32,
    green: i32,
    blue: i32,
}

fn get_cubes(draw: &str) -> CubeCounts {
    let cube_strings: Vec<&str> = draw.split(',').map(|x| x.trim()).collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube_string in cube_strings {
        let number_color: Vec<&str> = cube_string.split(' ').map(|x| x.trim()).collect();
        let number = number_color[0];
        let color = number_color[1];
        match color {
            "red" => red = number.parse().unwrap(),
            "green" => green = number.parse().unwrap(),
            "blue" => blue = number.parse().unwrap(),
            _ => panic!("Unknown color!"),
        }
    }

    CubeCounts { red, green, blue }
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
