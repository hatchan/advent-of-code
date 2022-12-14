fn main() {
    let input = include_str!("../input.txt");
    println!("Day 10 A: {}", solve_a(input));
    println!("Day 10 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    todo!()
}

fn solve_b(input: &str) -> usize {
    todo!()
}

enum Instruction {
    Noop,
    Addx(isize),
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
        todo!()
        // assert_eq!(solve_b(INPUT), 36);
    }
}
