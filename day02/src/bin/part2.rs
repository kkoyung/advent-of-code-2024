use day02::*;
use std::fs;

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_really_safe(nums))
        .count()
}

fn is_really_safe(nums: &[i32]) -> bool {
    let nums = Vec::from(nums);
    for i in 0..nums.len() {
        let mut nums = nums.clone();
        nums.remove(i);
        if is_safe(&nums[..]) {
            return true;
        }
    }
    false
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
    use day02::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
