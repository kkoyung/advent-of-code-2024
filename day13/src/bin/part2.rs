use day13::*;
use std::fs;

fn add_prize_distance(mut machine: Machine) -> Machine {
    machine.prize.0 += 10000000000000;
    machine.prize.1 += 10000000000000;
    machine
}

fn process(input: &str) -> usize {
    let machines = parse_input(input);

    machines
        .into_iter()
        .map(add_prize_distance)
        .map(|machine| match count(&machine) {
            Some(count) => cost(count),
            None => 0,
        })
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
    use day13::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
