fn main() {
    let input = include_str!("../input.txt");
    println!("Day 02 A: {}", solve_a(input));
    println!("Day 02 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    input.lines().enumerate().filter_map(filter_a).sum()
}

fn filter_a((i, line): (usize, &str)) -> Option<usize> {
    let (_, sets) = line.split_once(": ").expect("invalid format");
    for set in sets.split("; ") {
        for cubes in set.split(", ") {
            let (num, color) = cubes.split_once(" ").expect("invalid format");
            let num: usize = num.parse().expect("invalid format");
            if num > max_per_color(color) {
                return None;
            }
        }
    }

    Some(i + 1)
}

fn max_per_color(color: &str) -> usize {
    match color {
        "blue" => 14,
        "green" => 13,
        "red" => 12,
        _ => panic!("invalid color {color}"),
    }
}

fn solve_b(input: &str) -> usize {
    input.lines().map(filter_b).sum()
}

fn filter_b(line: &str) -> usize {
    let mut result_red = 0;
    let mut result_green = 0;
    let mut result_blue = 0;

    let (_, sets) = line.split_once(": ").expect("invalid format");
    for set in sets.split("; ") {
        let (red, green, blue) = parse_set(set);
        result_red = result_red.max(red);
        result_green = result_green.max(green);
        result_blue = result_blue.max(blue);
    }

    result_red * result_green * result_blue
}

fn parse_set(set: &str) -> (usize, usize, usize) {
    let mut result = (0, 0, 0);
    let (red, blue, green) = &mut result;

    for cubes in set.split(", ") {
        let (num, color) = cubes.split_once(" ").expect("invalid format");
        let num: usize = num.parse().expect("invalid format");
        match color {
            "blue" => *blue = num,
            "green" => *green = num,
            "red" => *red = num,
            _ => panic!("invalid color {color}"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 8);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 2286);
    }
}
