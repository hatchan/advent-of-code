use std::{collections::HashSet, iter};

fn main() {
    let input = include_str!("../input.txt");
    println!("Day 09 A: {}", solve_a(input));
    println!("Day 09 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    solve_n(input, 1)
}

fn solve_b(input: &str) -> usize {
    solve_n(input, 9)
}

fn solve_n(input: &str, knots: usize) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();

    let mut head = Coord::new();
    let mut rope: Vec<Coord> = iter::repeat(Coord::new()).take(knots).collect();

    for command in input.lines().map(Command::parse_line) {
        for _ in 0..command.distance {
            head.move_direction(command.direction);

            let mut prev_knot = &head;
            for knot in rope.iter_mut() {
                knot.follow(prev_knot);
                prev_knot = knot;
            }

            visited.insert(prev_knot.clone());
        }
    }

    visited.len()
}

struct Command {
    direction: (i32, i32),
    distance: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_direction(&mut self, direction: (i32, i32)) {
        self.x += direction.0;
        self.y += direction.1;
    }

    fn follow(&mut self, head: &Self) {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;
        let x_diff_abs = x_diff.abs();
        let y_diff_abs = y_diff.abs();

        if x_diff_abs > 1 || y_diff_abs > 1 {
            self.x += x_diff_abs.min(1) * x_diff.signum();
            self.y += y_diff_abs.min(1) * y_diff.signum();
        }
    }
}

impl Command {
    fn parse_line(line: &str) -> Self {
        let (direction, distance) = line.split_once(' ').expect("Invalid line");
        let direction = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid direction"),
        };
        let distance = distance.parse().expect("Invalid distance");

        Command {
            direction,
            distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");
    static INPUT2: &'static str = include_str!("../example2.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 13);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT2), 36);
    }
}
