fn main() {
    let input = include_str!("../input.txt");
    println!("Day 06 A: {}", solve_a(input));
    println!("Day 06 B: {}", solve_b(input));
}

// 1655
fn solve_a(input: &str) -> usize {
    let mut buf = vec![];

    for (i, c) in input.chars().enumerate() {
        buf.push(c);

        if buf.len() == 4 {
            if check_unique(&buf) {
                return i + 1;
            } else {
                buf.remove(0);
                // plus 1 since we're always one behind
            }
        }
    }

    unreachable!()
}

fn solve_b(input: &str) -> usize {
    let mut buf = vec![];

    for (i, c) in input.chars().enumerate() {
        buf.push(c);

        if buf.len() == 14 {
            if check_unique(&buf) {
                return i + 1;
            } else {
                buf.remove(0);
                // plus 1 since we're always one behind
            }
        }
    }

    unreachable!()
}

/// Returns true if is does not contain any duplicates
fn check_unique(buf: &Vec<char>) -> bool {
    for (i, char_left) in buf.iter().enumerate() {
        for char_right in buf.iter().skip(i + 1) {
            if char_left == char_right {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 7);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 19);
    }
}
