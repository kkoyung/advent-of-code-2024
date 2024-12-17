// use day16::*;
use std::fs;

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn process(input: &str) -> usize {
    let (_map, _start_position, _end_position) = parse_input(input);
    0
}

fn parse_input(input: &str) -> (Map, Position, Position) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let start_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
        .find(|(_position, character)| **character == 'S')
        .unwrap()
        .0;

    let end_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
        .find(|(_position, character)| **character == 'E')
        .unwrap()
        .0;

    (map, start_position, end_position)
}

// =====================================================================

fn main() {
    println!("Part 2");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::process;
    use day16::{INPUT2_1, INPUT2_2, OUTPUT2_1, OUTPUT2_2};

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT2_1), OUTPUT2_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT2_2), OUTPUT2_2);
    }
}
