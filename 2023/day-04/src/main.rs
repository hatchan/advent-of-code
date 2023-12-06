fn main() {
    let input = include_str!("../input.txt");
    println!("Day 04 A: {}", solve_a(input));
    // println!("Day 04 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    input.lines().map(calc_winnings).sum()
}

fn calc_winnings(input: &str) -> usize {
    let (_card, content) = input.split_once(": ").expect("invalid format");
    let (winnings, numbers) = content.split_once(" | ").expect("invalid format");
    let winnings: Vec<_> = winnings.split(' ').filter(|w| !w.is_empty()).collect();

    let wins = numbers
        .split(' ')
        .filter(|n| !n.is_empty() && winnings.contains(n))
        .count();

    if wins == 0 {
        return 0;
    } else {
        2_usize.pow((wins - 1) as u32)
    }
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
