fn main() {
    let input = include_str!("../input.txt");
    println!("Day 04 A: {}", solve_a(input));
    println!("Day 04 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(",").expect("no comma");

            let left = Range::parse(left);
            let right = Range::parse(right);

            left.contains(&right) || right.contains(&left)
        })
        .count()
}

fn solve_b(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(",").expect("no comma");

            let left = Range::parse(left);
            let right = Range::parse(right);

            left.has_overlap(&right)
        })
        .count()
}

struct Range {
    begin: usize,
    end: usize,
}

impl Range {
    fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    fn parse(input: &str) -> Self {
        let (begin, end) = input.split_once("-").expect("no dash");
        let begin = begin.parse().expect("not a number");
        let end = end.parse().expect("not a number");
        Self::new(begin, end)
    }

    fn contains(&self, other: &Self) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    fn has_overlap(&self, other: &Self) -> bool {
        self.begin <= other.end && other.begin <= self.end
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
        assert_eq!(solve_b(INPUT), 4);
    }
}
