// use day19::*;
use std::fs;

fn is_possible(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .any(|&pattern| is_possible(patterns, &design[pattern.len()..]))
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    let designs = lines.skip(1).collect::<Vec<&str>>();

    designs
        .iter()
        .filter(|design| is_possible(&patterns, design))
        .count()
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
    use day19::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
