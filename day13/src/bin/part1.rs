use day13::*;
use std::fs;

fn process(input: &str) -> usize {
    let machines = parse_input(input);

    machines
        .iter()
        .map(|machine| match count(machine) {
            Some(count) => cost(count),
            None => 0,
        })
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
    use day13::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
