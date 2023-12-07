use std::str::Lines;

fn main() {
    let input = include_str!("../input.txt");
    println!("Day 05 A: {}", solve_a(input));
    println!("Day 05 B: {}", solve_b(input));
}

fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();

    // First line should the the seed inputs, so just parse that differently
    let seeds = parse_seeds_a(&mut lines);
    let maps = parse_maps(&mut lines);

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

fn parse_seeds_a(lines: &mut Lines) -> Vec<usize> {
    lines
        .next()
        .expect("no seeds found")
        .split_once(": ")
        .map(|(_, seeds)| {
            seeds
                .split(' ')
                .map(|c| c.parse().expect("expecting a number"))
                .collect()
        })
        .expect("invalid format")
}

fn solve_b(input: &str) -> usize {
    let mut lines = input.lines();

    // First line should the the seed inputs, so just parse that differently
    let seeds = parse_seeds_b(&mut lines);
    let maps = parse_maps(&mut lines);

    seeds
        .into_iter()
        .map(|(start, range)| {
            let mut current = create_vec(start, range);
            for map in &maps {
                map.translate_range(&mut current);
            }
            current
        })
        .flatten()
        .min()
        .expect("a number is expected")
}

fn create_vec(start: usize, range: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(range);

    for i in start..(start + range) {
        result.push(i);
    }

    result
}

fn parse_seeds_b(lines: &mut Lines) -> Vec<(usize, usize)> {
    lines
        .next()
        .expect("no seeds found")
        .split_once(": ")
        .map(|(_, seeds)| {
            let seeds: Vec<_> = seeds
                .split(' ')
                .map(|c| c.parse().expect("expecting a number"))
                .collect();
            seeds.chunks(2).map(|c| (c[0], c[1])).collect()
        })
        .expect("invalid format")
}

fn parse_maps(lines: &mut Lines) -> Vec<Map> {
    let mut buf = vec![];
    // Now we will parse all the mappings
    for line in lines {
        if line.ends_with(':') {
            // this the header, add a new section
            buf.push(Map::new());
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

        buf.last_mut()
            .expect("should always have a last section")
            .add_mapping(mapping);
    }

    buf
}

struct Map {
    mappings: Vec<Mappings>,
}

impl Map {
    fn new() -> Self {
        Self { mappings: vec![] }
    }

    fn add_mapping(&mut self, mapping: Mappings) {
        self.mappings.push(mapping);
    }

    fn translate(&self, input: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.translate(input))
            .unwrap_or(input)
    }

    fn translate_range(&self, input: &mut Vec<usize>) {
        input.iter_mut().for_each(|i| {
            *i = self.translate(*i);
        });
    }
}

struct Mappings {
    destination_start: usize,

    source_start: usize,
    source_end: usize,

    range_length: usize,
}

impl Mappings {
    fn new(destination_start: usize, source_start: usize, range_length: usize) -> Self {
        Self {
            destination_start,
            source_start,
            source_end: source_start + range_length,
            range_length,
        }
    }

    /// Converts input, if there is a mapping possible, otherwise return None.
    fn translate(&self, input: usize) -> Option<usize> {
        if self.is_contained(input) {
            let offset = input - self.source_start;
            let result = self.destination_start + offset;
            Some(result)
        } else {
            // it is not contained
            None
        }
    }

    /// Converts input, if there is a mapping possible, otherwise return None.
    fn translate_range(&self, input: usize) -> Option<usize> {
        if self.is_contained(input) {
            let offset = input - self.source_start;
            let result = self.destination_start + offset;
            Some(result)
        } else {
            // it is not contained
            None
        }
    }

    fn is_contained(&self, input: usize) -> bool {
        input >= self.source_start && input <= self.source_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn solve_a_success() {
        assert_eq!(solve_a(INPUT), 35);
    }

    #[test]
    fn solve_b_success() {
        assert_eq!(solve_b(INPUT), 46);
    }
}
