use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let input_path = {
        // Just assume the first argument is the path to the file
        let args: Vec<String> = env::args().collect();
        match args.len() {
            1 => panic!("No arguments given"),
            2 => args[1].clone(),
            _ => panic!("Too many arguments given"),
        }
    };

    let f = File::open(input_path).expect("Unable to open input file");
    let reader = BufReader::new(f);

    let total: u64 = reader.lines().map(parse_line).sum();

    println!("Total {}", total);
}

fn parse_line(input: Result<String, Error>) -> u64 {
    let mut r = 0;
    let (expected, action) = input
        .expect("invalid line")
        .split_once(" ")
        .map(|(expected, action)| (parse_choice(expected), parse_choice(action)))
        .expect("invalid format");

    r += match action {
        Choices::Rock => 1,
        Choices::Paper => 2,
        Choices::Scissors => 3,
    };

    r += if expected == action {
        3
    } else {
        match (expected, action) {
            (Choices::Rock, Choices::Paper) => 6,
            (Choices::Paper, Choices::Scissors) => 6,
            (Choices::Scissors, Choices::Rock) => 6,
            _ => 0,
        }
    };

    r
}

fn parse_choice(input: &str) -> Choices {
    match input {
        "A" | "X" => Choices::Rock,
        "B" | "Y" => Choices::Paper,
        "C" | "Z" => Choices::Scissors,
        _ => panic!("invalid input: {}", input),
    }
}

#[derive(PartialEq)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

// A rock
// B paper
// C scissors

// X rock
// Y paper
// Z scissors
