fn main() {
    let input = include_str!("../input.txt");
    println!("Day 01 A: {}", solve_a(input));
    println!("Day 01 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    input.lines().map(parse_line_a).sum()
}

fn solve_b(input: &str) -> usize {
    input.lines().map(parse_line_b).sum()
}

fn parse_line_a(input: impl Into<String>) -> usize {
    let mut left = '0';
    let mut right = '0';

    for c in input.into().chars() {
        if is_number(c) {
            if left == '0' {
                left = c;
            }
            right = c;
        }
    }

    format!("{left}{right}")
        .parse()
        .expect("should be parsable")
}

fn is_number(c: char) -> bool {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn parse_line_b(line: &str) -> usize {
    let mut buf = vec![];

    for (i, c) in line.char_indices() {
        if let '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' = c {
            buf.push(&line[i..i + 1]);
            continue;
        }

        let m = &line[i..];
        if m.starts_with("one") {
            buf.push("1");
        } else if m.starts_with("two") {
            buf.push("2");
        } else if m.starts_with("three") {
            buf.push("3");
        } else if m.starts_with("four") {
            buf.push("4");
        } else if m.starts_with("five") {
            buf.push("5");
        } else if m.starts_with("six") {
            buf.push("6");
        } else if m.starts_with("seven") {
            buf.push("7");
        } else if m.starts_with("eight") {
            buf.push("8");
        } else if m.starts_with("nine") {
            buf.push("9");
        }
    }

    format!("{}{}", buf[0], buf[buf.len() - 1])
        .parse()
        .expect("should be parsable")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_A: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT_A), 142);
    }

    static INPUT_B: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT_B), 281);
    }
}
