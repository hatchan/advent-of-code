fn main() {
    let input = include_str!("../input.txt");
    println!("Day 03 A: {}", solve_a(input));
    println!("Day 03 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    input.lines().map(parse_line_a).sum()
}

fn parse_line_a(line: &str) -> u64 {
    let (left, right) = line.split_at(line.len() / 2);

    for left_c in left.chars() {
        for right_c in right.chars() {
            if left_c == right_c {
                return char_to_uint(left_c);
            }
        }
    }

    unreachable!()
}

fn solve_b(input: &str) -> u64 {
    let mut result = 0;
    let mut i = 0;
    let mut buf = vec![];

    for line in input.lines() {
        buf.push(line.chars().collect::<Vec<_>>());

        if i == 2 {
            // On the third line, calculate the char that is on every lines
            let unique_1 = &buf[0];
            let unique_2 = &buf[1];
            let unique_3 = &buf[2];

            for c in unique_1 {
                if unique_2.contains(c) && unique_3.contains(c) {
                    result += char_to_uint(*c);
                    break;
                }
            }

            buf.clear();
            i = 0;
        } else {
            // Otherwise just increment the counter
            i += 1;
        }
    }

    result
}

fn char_to_uint(c: char) -> u64 {
    if c.is_lowercase() {
        return c as u64 - 96;
    } else {
        return c as u64 - 38;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 157);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 70);
    }

    #[test]
    fn char_to_uint_success() {
        assert_eq!(char_to_uint('a'), 1);
        assert_eq!(char_to_uint('z'), 26);
        assert_eq!(char_to_uint('A'), 27);
        assert_eq!(char_to_uint('Z'), 52);
    }
}
