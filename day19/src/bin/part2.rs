// use day19::*;
use std::fs;

fn number_of_ways(patterns: &[&str], design: &str) -> usize {
    let mut dp: Vec<usize> = vec![0; design.len() + 1];
    dp[0] = 1;

    for length in 1..=design.len() {
        let substring = &design[0..length];
        for pattern in patterns {
            if substring.ends_with(pattern) {
                dp[length] += dp[length-pattern.len()];
            }
        }
    }

    dp[design.len()]
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    let designs = lines.skip(1).collect::<Vec<&str>>();

    designs
        .iter()
        .map(|design| number_of_ways(&patterns, design))
        .sum()
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
    use day19::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
