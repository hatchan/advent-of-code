use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Day 05 A: {}", solve_a(input));
    println!("Day 05 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> String {
    let (mut state, operations) = parse(input);

    for operation in operations {
        for _ in 0..operation.amount {
            let tmp = state[operation.from - 1].pop().expect("no element found ");
            state[operation.to - 1].push(tmp);
        }
    }

    let mut result = String::new();
    for stack in state {
        result.push(stack.last().cloned().unwrap_or(' '))
    }

    result
}

fn solve_b(input: &str) -> String {
    todo!()
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Operation>) {
    let (state, operations) = input.split_once("\n\n").expect("no empty line found");

    let mut state_buf = vec![];

    let mut state_iter = state.lines().rev();
    if let Some(first_line) = state_iter.next() {
        let num_stacks: usize = first_line
            .split(" ")
            .last()
            .expect("no stacks found")
            .parse()
            .expect("invalid number");
        for _ in 1..num_stacks + 1 {
            state_buf.push(vec![]);
        }
    }

    while let Some(line) = state_iter.next() {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    state_buf[i].push(c);
                }
            });
    }

    let operations = operations.lines().map(Operation::parse_operation).collect();

    (state_buf, operations)
}

#[derive(Debug, PartialEq, Eq)]
struct Operation {
    amount: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn new(amount: usize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }

    fn parse_operation(line: &str) -> Self {
        let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();

        let capture = re.captures(line).expect("invalid format");
        let amount = capture[1].parse().expect("invalid amount");
        let from = capture[2].parse().expect("invalid from");
        let to = capture[3].parse().expect("invalid to");

        Self::new(amount, from, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), "CMZ");
    }

    #[test]
    fn solve_b_success() {
        panic!();
        // assert_eq!(solve_b(INPUT), 70);
    }

    #[test]
    fn parse_input() {
        let expected_state = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected_operations = vec![
            Operation::new(1, 2, 1),
            Operation::new(3, 1, 3),
            Operation::new(2, 2, 1),
            Operation::new(1, 1, 2),
        ];
        let (state, operations) = parse(INPUT);

        assert_eq!(state, expected_state);
        assert_eq!(operations, expected_operations);
    }
}
