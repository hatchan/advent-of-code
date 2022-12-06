fn main() {
    let input = include_str!("../input.txt");
    println!("Day 04 A: {}", solve_a(input));
    println!("Day 04 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").expect("no comma");

            let left = Range::parse(left);
            let right = Range::parse(right);

            if (left.begin <= right.begin && left.end >= right.end)
                || right.begin <= left.begin && right.end >= left.end
            {
                1
            } else {
                0
            }
        })
        .sum()
}
fn solve_b(input: &str) -> u64 {
    todo!()
}

struct Range {
    begin: u64,
    end: u64,
}

impl Range {
    fn new(begin: u64, end: u64) -> Self {
        Self { begin, end }
    }

    fn parse(input: &str) -> Self {
        let (begin, end) = input.split_once("-").expect("no dash");
        let begin = begin.parse().expect("not a number");
        let end = end.parse().expect("not a number");
        Self::new(begin, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 2);
    }

    #[test]
    fn solve_b_success() {
        panic!();
        // assert_eq!(solve_b(INPUT), 70);
    }
}
