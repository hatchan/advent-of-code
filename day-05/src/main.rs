fn main() {
    let input = include_str!("../input.txt");
    println!("Day 05 A: {}", solve_a(input));
    println!("Day 05 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> String {
    todo!()
}
fn solve_b(input: &str) -> String {
    todo!()
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
}
