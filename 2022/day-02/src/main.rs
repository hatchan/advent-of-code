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
    let (expected, result) = input
        .expect("invalid line")
        .split_once(" ")
        .map(|(expected, action)| (parse_choice(expected), parse_game_result(action)))
        .expect("invalid format");

    let action = match result {
        GameResult::Win => match expected {
            Choices::Rock => Choices::Paper,
            Choices::Paper => Choices::Scissors,
            Choices::Scissors => Choices::Rock,
        },
        GameResult::Draw => expected,
        GameResult::Lose => match expected {
            Choices::Rock => Choices::Scissors,
            Choices::Paper => Choices::Rock,
            Choices::Scissors => Choices::Paper,
        },
    };

    action as u64 + result as u64
}

fn parse_choice(input: &str) -> Choices {
    match input {
        "A" => Choices::Rock,
        "B" => Choices::Paper,
        "C" => Choices::Scissors,
        _ => panic!("invalid input: {}", input),
    }
}

fn parse_game_result(input: &str) -> GameResult {
    match input {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("invalid input: {}", input),
    }
}

#[derive(PartialEq)]
enum Choices {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
