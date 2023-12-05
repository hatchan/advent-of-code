fn main() {
    let input = include_str!("../input.txt");
    println!("Day 03 A: {}", solve_a(input));
    println!("Day 03 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let (part_numbers, parts) = parse_input(input);

    part_numbers
        .into_iter()
        .filter(|p| {
            parts
                .iter()
                .any(|(row, column, _)| p.is_next_to(*row, *column))
        })
        .map(|p| p.value)
        .sum()
}

fn parse_input(input: &str) -> (Vec<PartNumber>, Vec<(isize, isize, char)>) {
    let mut part_numbers = vec![];
    let mut parts = vec![];

    let mut buf = String::new();
    let mut start = None;

    for (row, line) in input.lines().enumerate() {
        let mut max_column = 0;
        for (column, c) in line.char_indices() {
            match c {
                '.' => {
                    if let Some(s) = start.take() {
                        part_numbers.push(PartNumber {
                            column: s,
                            row: row as isize,
                            until: column as isize - 1,
                            value: buf.parse().expect("expected a number"),
                        });
                        buf.clear();
                    }
                }
                '0'..='9' => {
                    if start.is_none() {
                        start.replace(column as isize);
                    }

                    buf.push(c)
                }
                _ => {
                    if let Some(s) = start.take() {
                        part_numbers.push(PartNumber {
                            column: s,
                            row: row as isize,
                            until: column as isize - 1,
                            value: buf.parse().expect("expected a number"),
                        });
                        buf.clear();
                    }
                    parts.push((row as isize, column as isize, c))
                }
            }
            max_column = column;
        }

        if let Some(s) = start.take() {
            part_numbers.push(PartNumber {
                column: s,
                row: row as isize,
                until: max_column as isize,
                value: buf.parse().expect("expected a number"),
            });
        }

        buf.clear();
    }

    (part_numbers, parts)
}

#[derive(Debug)]
struct PartNumber {
    column: isize,
    row: isize,
    until: isize,
    value: usize,
}

impl PartNumber {
    fn is_next_to(&self, row: isize, column: isize) -> bool {
        (self.row - 1 <= row && self.row + 1 >= row)
            && (self.column - 1 <= column && self.until + 1 >= column)
    }
}

fn solve_b(input: &str) -> usize {
    let (part_numbers, parts) = parse_input(input);

    parts
        .into_iter()
        .filter_map(|(row, column, c)| {
            if c != '*' {
                return None;
            }

            let multiply_adjacent_part_numbers =
                multiply_adjacent_part_numbers(row, column, &part_numbers);

            Some(multiply_adjacent_part_numbers)
        })
        .sum()
}

fn multiply_adjacent_part_numbers(row: isize, column: isize, part_numbers: &[PartNumber]) -> usize {
    let results: Vec<_> = part_numbers
        .iter()
        .filter_map(|p| {
            if p.is_next_to(row, column) {
                Some(p.value)
            } else {
                None
            }
        })
        .collect();

    match results.len() {
        0 | 1 => 0,
        2 => results[0] * results[1],
        _ => panic!("expected only max two part numbers"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 4361);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 467835);
    }
}
