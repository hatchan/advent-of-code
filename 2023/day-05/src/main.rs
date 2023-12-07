fn main() {
    let input = include_str!("../input.txt");
    println!("Day 05 A: {}", solve_a(input));
    // println!("Day 05 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();

    // First line should the the seed inputs, so just parse that differently
    let seeds: Vec<usize> = lines
        .next()
        .expect("no seeds found")
        .split_once(": ")
        .map(|(_, seeds)| {
            seeds
                .split(' ')
                .map(|c| c.parse().expect("expecting a number"))
                .collect()
        })
        .expect("invalid format");

    let mut maps: Vec<Map> = vec![];

    // Now we will parse all the mappings
    for line in lines {
        if line.ends_with(':') {
            // this the header, add a new section
            maps.push(Map::new());
            continue;
        }

        if line.is_empty() {
            // Nothing to do here.
            continue;
        }

        // This should be a mapping rule, so parse it and add it to our current map
        let mapping = {
            let (destination_start, remainder) = line.split_once(' ').expect("invalid format");
            let (source_start, range_length) = remainder.split_once(' ').expect("invalid format");
            Mappings::new(
                destination_start.parse().expect("expected a number"),
                source_start.parse().expect("expected a number"),
                range_length.parse().expect("expected a number"),
            )
        };

        maps.last_mut()
            .expect("should always have a last section")
            .add_mapping(mapping);
    }

    seeds
        .into_iter()
        .map(|seed| {
            let mut current = seed;
            for map in &maps {
                current = map.translate(current);
            }
            current
        })
        .min()
        .expect("a number is expected")
}

struct Map {
    mappings: Vec<Mappings>,
}

impl Map {
    fn new() -> Self {
        Self { mappings: vec![] }
    }

    fn translate(&self, input: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.translate(input))
            .unwrap_or(input)
    }

    fn add_mapping(&mut self, mapping: Mappings) {
        self.mappings.push(mapping);
    }
}

struct Mappings {
    destination_start: usize,
    // destination_end: usize,
    source_start: usize,
    source_end: usize,
    // range_length: usize,
}

impl Mappings {
    fn new(destination_start: usize, source_start: usize, range_length: usize) -> Self {
        Self {
            destination_start,
            // destination_end: destination_start + range_length,
            source_start,
            source_end: source_start + range_length,
            // range_length,
        }
    }

    /// Converts input, if there is a mapping possible, otherwise return None.
    fn translate(&self, input: usize) -> Option<usize> {
        if input >= self.source_start && input <= self.source_end {
            let offset = input - self.source_start;
            let result = self.destination_start + offset;
            Some(result)
        } else {
            // it is not contained
            None
        }
    }
}

// fn solve_b(input: &str) -> usize {
//     todo!("solve_b")
// }

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 35);
    }

    // #[test]
    // fn solve_b_success() {
    //     assert_eq!(solve_b(INPUT), 30);
    // }
}
