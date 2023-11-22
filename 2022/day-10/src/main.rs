fn main() {
    let input = include_str!("../input.txt");
    println!("Day 10 A: {}", solve_a(input));
    println!("Day 10 B:\n{}", solve_b(input));
}

fn solve_a(input: &str) -> isize {
    let mut instructions = input.lines().map(Instruction::parse);

    let mut result = 0;

    let mut current_cycle = 1;

    let mut current_instruction = None;
    let mut cycle_remaining = 1;
    let mut current_state = 1;

    loop {
        if cycle_remaining == 0 {
            if let Some(instruction) = current_instruction.take() {
                match instruction {
                    Instruction::Noop => {}
                    Instruction::Add(value) => {
                        current_state += value;
                    }
                }
            }
        }

        if (current_cycle - 20) % 40 == 0 {
            println!("{}: {} ({})", current_cycle, current_state, result);
            result += current_state * current_cycle;
        }

        if current_instruction.is_none() {
            match instructions.next() {
                Some(instruction) => {
                    cycle_remaining = instruction.cycles();
                    current_instruction = Some(instruction);
                }
                None => break,
            };
        }

        cycle_remaining -= 1;
        current_cycle += 1;
    }

    result
}

fn solve_b(input: &str) -> String {
    let mut instructions = input.lines().map(Instruction::parse);

    let mut crt_row = vec![];

    let mut current_cycle = 1;

    let mut cycle_remaining = 1;
    let mut current_instruction = None;
    let mut register = 1;

    loop {
        println!("{}: {}", current_cycle, register);
        if cycle_remaining == 0 {
            if let Some(instruction) = current_instruction.take() {
                match instruction {
                    Instruction::Noop => {}
                    Instruction::Add(value) => {
                        register += value;
                    }
                }
            }
        }

        if current_instruction.is_none() {
            match instructions.next() {
                Some(instruction) => {
                    cycle_remaining = instruction.cycles();
                    current_instruction = Some(instruction);
                }
                None => break,
            };
        }

        let crt_cycle = current_cycle % 40;
        if crt_cycle == register - 1 || crt_cycle == register || crt_cycle == register + 1 {
            crt_row.push('#');
        } else {
            crt_row.push('.');
        }

        cycle_remaining -= 1;
        current_cycle += 1;
    }

    crt_row
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

enum Instruction {
    Noop,
    Add(isize),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        if input == "noop" {
            Instruction::Noop
        } else {
            let (_, value) = input.split_once(" ").unwrap();
            Instruction::Add(value.parse().unwrap())
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 13140);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), include_str!("../example.solution.txt"));
    }
}
