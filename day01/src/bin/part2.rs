// use day01::*;
use std::{collections::HashMap, fs};

fn process(input: &str) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|v| v.parse::<usize>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    });

    let mut hash: HashMap<usize, usize> = HashMap::new();
    right.iter().for_each(|&v| {
        hash.insert(v, *hash.get(&v).unwrap_or(&0) + 1);
    });

    left.iter()
        .map(|&v| v * (*hash.get(&v).unwrap_or(&0)))
        .sum::<usize>()
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
    use day01::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
