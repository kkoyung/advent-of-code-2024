// use day01::*;
use std::fs;

fn process(input: &str) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|v| v.parse::<usize>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    });

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(left_var, right_var)| left_var.abs_diff(right_var))
        .sum()
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
    use day01::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
