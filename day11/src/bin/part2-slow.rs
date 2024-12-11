use day11::*;
use std::fs;

fn expand(stone: u64, blinks: usize) -> usize {
    if stone == 0 && blinks >= 4 {
        [2, 0, 2, 4]
            .iter()
            .map(|&stone| expand(stone, blinks - 4))
            .sum()
    } else if stone == 1 && blinks >= 3 {
        [2, 0, 2, 4]
            .iter()
            .map(|&stone| expand(stone, blinks - 3))
            .sum()
    } else if stone == 2 && blinks >= 3 {
        [4, 0, 4, 8]
            .iter()
            .map(|&stone| expand(stone, blinks - 3))
            .sum()
    } else if stone == 3 && blinks >= 3 {
        [6, 0, 7, 2]
            .iter()
            .map(|&stone| expand(stone, blinks - 3))
            .sum()
    } else if stone == 4 && blinks >= 3 {
        [8, 0, 9, 6]
            .iter()
            .map(|&stone| expand(stone, blinks - 3))
            .sum()
    } else if stone == 5 && blinks >= 5 {
        [2, 0, 4, 8, 2, 8, 8, 0]
            .iter()
            .map(|&stone| expand(stone, blinks - 5))
            .sum()
    } else if stone == 6 && blinks >= 5 {
        [2, 4, 5, 7, 9, 4, 5, 6]
            .iter()
            .map(|&stone| expand(stone, blinks - 5))
            .sum()
    } else if stone == 7 && blinks >= 5 {
        [2, 8, 6, 7, 6, 0, 3, 2]
            .iter()
            .map(|&stone| expand(stone, blinks - 5))
            .sum()
    } else if stone == 8 && blinks >= 5 {
        [3, 2, 7, 7, 2, 6, 16192]
            .iter()
            .map(|&stone| expand(stone, blinks - 5))
            .sum()
    } else if stone == 9 && blinks >= 5 {
        [3, 6, 8, 6, 9, 1, 8, 4]
            .iter()
            .map(|&stone| expand(stone, blinks - 5))
            .sum()
    } else if blinks > 0 {
        apply_rules(stone)
            .iter()
            .map(|&stone| expand(stone, blinks - 1))
            .sum()
    } else {
        1
    }
}

fn process(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|number_in_text| number_in_text.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    stones.iter().map(|&stone| expand(stone, 75)).sum()
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
    use day11::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
