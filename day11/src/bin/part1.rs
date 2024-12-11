use day11::*;
use std::fs;

fn process(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|number_in_text| number_in_text.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|stone| apply_rules(*stone))
            .collect::<Vec<u64>>();
    }
    stones.len()
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
    use day11::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
