use day22::*;
use std::fs;

fn nth(mut secret: usize, n: usize) -> usize {
    for _ in 0..n {
        secret = next(secret);
    }
    secret
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|text| text.parse::<usize>().unwrap())
        .map(|secret| nth(secret, 2000))
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
    use day22::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
