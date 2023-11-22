use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut all_elves = vec![];
    let mut current_elf = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            all_elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<u64>().expect("Unable to parse line");
        }
    }

    all_elves.sort();

    let result = all_elves[all_elves.len() - 1]
        + all_elves[all_elves.len() - 2]
        + all_elves[all_elves.len() - 3];

    println!("Max {}", result);
}
