fn main() {
    let input = include_str!("../input.txt");
    println!("Day 08 A: {}", solve_a(input));
    println!("Day 08 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let height_map = parse_height_map(input);
    let mut visible_map = create_visible_map(&height_map);

    let total_rows = height_map.len();
    let total_columns = height_map[0].len();

    // iterate throw through all rows
    for row in 0..total_rows {
        let mut max = -1;
        for column in 0..total_columns {
            if height_map[row][column] > max {
                max = height_map[row][column];
                visible_map[row][column] = true;
            }
        }

        let mut max = -1;
        for column in (0..total_columns).rev() {
            if height_map[row][column] > max {
                max = height_map[row][column];
                visible_map[row][column] = true;
            }
        }
    }

    // iterate throw through all cols
    for column in 0..total_columns {
        let mut max = -1;
        for row in 0..total_rows {
            if height_map[row][column] > max {
                max = height_map[row][column];
                visible_map[row][column] = true;
            }
        }

        let mut max = -1;
        for row in (0..total_rows).rev() {
            if height_map[row][column] > max {
                max = height_map[row][column];
                visible_map[row][column] = true;
            }
        }
    }

    visible_map
        .iter()
        .map(|row| row.iter().filter(|visible| **visible).count())
        .sum()
}

fn solve_b(input: &str) -> usize {
    let mut highest_scenic_score = 0;
    let height_map = parse_height_map(input);

    let total_rows = height_map.len();
    let total_columns = height_map[0].len();

    for row in 0..total_rows {
        for column in 0..total_columns {
            let current_height = height_map[row][column];

            // 0..r
            let left = trees_in_sight(
                &height_map,
                current_height,
                (0..column).rev().map(|col| (row, col)),
            );

            // r..rows
            let right = trees_in_sight(
                &height_map,
                current_height,
                (column + 1..total_columns).map(|col| (row, col)),
            );

            // 0..c
            let top = trees_in_sight(
                &height_map,
                current_height,
                (0..row).rev().map(|row| (row, column)),
            );

            // c..cols
            let bottom = trees_in_sight(
                &height_map,
                current_height,
                (row + 1..total_rows).map(|row| (row, column)),
            );

            let scenic_score = left * right * top * bottom;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

fn trees_in_sight(
    height_map: &Vec<Vec<isize>>,
    current_height: isize,
    row: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let mut buf = 0;

    for (row, column) in row {
        buf += 1;

        if height_map[row][column] >= current_height {
            break;
        }
    }

    buf
}

fn create_visible_map(height_map: &Vec<Vec<isize>>) -> Vec<Vec<bool>> {
    let total_rows = height_map.len();
    let total_columns = height_map[0].len();

    (0..total_rows)
        .map(|_| (0..total_columns).map(|_| false).collect())
        .collect()
}

fn parse_height_map(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 21);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 8);
    }
}
