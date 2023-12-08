fn main() {
    let input = include_str!("../input.txt");
    println!("Day 06 A: {}", solve_a(input));
    println!("Day 06 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();

    let times = lines.next().expect("invalid format");
    let (_header, times) = times.split_once(':').expect("invalid format");
    let times: Vec<usize> = times
        .split_whitespace()
        .map(|t| t.parse().expect("invalid number"))
        .collect();

    let distances = lines.next().expect("invalid format");
    let (_header, distances) = distances.split_once(':').expect("invalid format");
    let distances: Vec<usize> = distances
        .split_whitespace()
        .map(|t| t.parse().expect("invalid number"))
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(calc_records)
        .product()
}

fn calc_records((time, distance): (&usize, &usize)) -> usize {
    let first = (1..*time)
        .find(|i| {
            let distance_traveled = (time - i) * i;
            distance_traveled > *distance
        })
        .expect("should have record");

    let last = (1..*time)
        .rev()
        .find(|i| {
            let distance_traveled = (time - i) * i;
            distance_traveled > *distance
        })
        .expect("should have record");

    (last - first) + 1
}

fn solve_b(input: &str) -> usize {
    let mut lines = input.lines();

    let times = lines.next().expect("invalid format");
    let (_header, times) = times.split_once(':').expect("invalid format");
    let times: Vec<&str> = times.split_whitespace().collect::<Vec<&str>>();
    let time: usize = times.join("").parse().expect("expected number");

    let distances = lines.next().expect("invalid format");
    let (_header, distances) = distances.split_once(':').expect("invalid format");
    let distances: Vec<&str> = distances.split_whitespace().collect::<Vec<&str>>();
    let distance: usize = distances.join("").parse().expect("expected number");

    calc_records((&time, &distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 288);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 71503);
    }
}
