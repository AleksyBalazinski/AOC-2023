use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };

    let lines = read_lines(&filename);
    let calibration_value = calibration_value_pt1(lines);

    println!("{calibration_value}");
}

fn calibration_value_pt1(lines: Vec<String>) -> i32 {
    let mut acc = 0;
    let mut number_str = String::new();
    for line in lines {
        match first_and_last_digit(&line) {
            Some((first, last)) => {
                number_str.clear();

                number_str.push(first);
                number_str.push(last);

                acc += number_str
                    .parse::<i32>()
                    .expect("This string doesn't represent a number");
            }
            None => continue,
        }
    }

    acc
}

fn first_and_last_digit(line: &str) -> Option<(char, char)> {
    let mut first: char = '\0';
    let mut last: char = '\0';
    for c in line.chars() {
        if c.is_numeric() {
            if first == '\0' {
                first = c;
                last = c;
            } else {
                last = c;
            }
        }
    }

    if first == '\0' {
        None
    } else {
        Some((first, last))
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
