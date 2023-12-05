fn main() {
    let input = include_str!("../input.txt");
    println!("Day 04 A: {}", solve_a(input));
    // println!("Day 04 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    todo!("solve_a")
}

fn solve_b(input: &str) -> usize {
    todo!("solve_b")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 13);
    }

    // #[test]
    // fn solve_b_success() {
    //     assert_eq!(solve_b(INPUT), 0);
    // }
}
