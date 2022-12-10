use std::{collections::HashMap, path::PathBuf};

fn main() {
    let input = include_str!("../input.txt");
    println!("Day 07 A: {}", solve_a(input));
    println!("Day 07 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let tree = parse_tree(input);

    tree.values().filter(|size| **size <= 100000).sum()
}

fn solve_b(input: &str) -> usize {
    let total = 70000000;
    let required = 30000000;

    let tree = parse_tree(input);

    let in_use = tree.get(&PathBuf::from("/")).unwrap();
    let unused = total - *in_use;
    let target = required - unused;

    *tree.values().filter(|size| **size >= target).min().unwrap()
}

fn parse_tree(input: &str) -> HashMap<PathBuf, usize> {
    let commands = input.lines().map(parse_line);

    let mut tree = HashMap::new();
    let mut current_directory = PathBuf::from("/");

    for command in commands {
        match command {
            Output::ChangeDirectory(directory) => match directory {
                Directory::Root => current_directory = PathBuf::from("/"),
                Directory::Up => {
                    current_directory.pop();
                }
                Directory::Name(name) => current_directory.push(name),
            },
            Output::File(_, size) => {
                let mut dir = current_directory.clone();
                *tree.entry(dir.clone()).or_default() += size;

                while dir.pop() {
                    *tree.entry(dir.clone()).or_default() += size;
                }
            }
            Output::Ls | Output::Directory(_) => (),
        }
    }

    tree
}

fn parse_line(line: &str) -> Output {
    if line.starts_with("$ cd") {
        let directory = match &line[5..] {
            "/" => Directory::Root,
            ".." => Directory::Up,
            name => Directory::Name(name.to_string()),
        };
        Output::ChangeDirectory(directory)
    } else if line.starts_with("$ ls") {
        Output::Ls
    } else if line.starts_with("dir ") {
        Output::Directory(line[4..].to_string())
    } else {
        let (size, name) = line.split_once(" ").expect("invalid file format");
        Output::File(name.to_owned(), size.parse().expect("invalid size"))
    }
}

enum Output {
    ChangeDirectory(Directory),
    Ls,
    Directory(String),
    File(String, usize),
}

enum Directory {
    Root,
    Up,
    Name(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 95437);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 24933642);
    }
}
