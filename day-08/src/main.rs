fn main() {
    let input = include_str!("../input.txt");
    println!("Day 08 A: {}", solve_a(input));
    println!("Day 08 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    todo!()
}

fn solve_b(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 21);
    }

    #[test]
    fn solve_b_success() {
        todo!()
        // assert_eq!(solve_b(INPUT), 21);
    }
}
