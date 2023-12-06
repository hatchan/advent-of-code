use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("Day 04 A: {}", solve_a(input));
    println!("Day 04 B: {}", solve_b(input));
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
        0
    } else {
        2_usize.pow((wins - 1) as u32)
    }
}

fn solve_b(input: &str) -> usize {
    let mut copies = 0;
    let mut look_ahead: VecDeque<usize> = VecDeque::new();

    for line in input.lines() {
        let (_card, content) = line.split_once(": ").expect("invalid format");
        let (winnings, numbers) = content.split_once(" | ").expect("invalid format");
        let winnings: Vec<_> = winnings.split(' ').filter(|w| !w.is_empty()).collect();

        let wins = numbers
            .split(' ')
            .filter(|n| !n.is_empty() && winnings.contains(n))
            .count();

        // Take the copies that we accumulated so far and add one for the current card
        let total_cards = look_ahead.pop_front().unwrap_or_default() + 1;

        for i in 0..wins {
            match look_ahead.get_mut(i) {
                Some(look_ahead_number) => *look_ahead_number += total_cards,
                None => {
                    look_ahead.push_back(total_cards);
                }
            }
        }

        copies += total_cards;
    }

    copies
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 13);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 30);
    }
}
