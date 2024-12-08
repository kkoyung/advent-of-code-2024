use day04::*;
use std::fs;

const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

fn count(states: &[Vec<u8>]) -> usize {
    states
        .iter()
        .flatten()
        .map(|state| {
            [
                if state & NORTH > 0 { 1 } else { 0 },
                if state & NORTH_WEST > 0 { 1 } else { 0 },
                if state & WEST > 0 { 1 } else { 0 },
                if state & SOUTH_WEST > 0 { 1 } else { 0 },
                if state & SOUTH > 0 { 1 } else { 0 },
                if state & SOUTH_EAST > 0 { 1 } else { 0 },
                if state & EAST > 0 { 1 } else { 0 },
                if state & NORTH_EAST > 0 { 1 } else { 0 },
            ]
            .iter()
            .sum::<usize>()
        })
        .sum()
}

fn process(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut states: Vec<Vec<u8>> = first(&map, TARGET[0]);
    for target in &TARGET[1..] {
        states = step(&map, &states, *target);
    }

    count(&states)
}

// =====================================================================

fn main() {
    println!("Part 1");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::process;
    use day04::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
