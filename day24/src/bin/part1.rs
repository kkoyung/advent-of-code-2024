use day24::*;
use std::fs;

fn process(input: &str) -> usize {
    let (_wires, gates, init_assignment) = parse_input(input);
    let (assignment, _gate_sequence) = run_gates(&gates, &init_assignment);
    wires_to_number(&assignment, "z")
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
    use day24::{INPUT1_1, INPUT1_2, OUTPUT1_1, OUTPUT1_2};

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT1_1), OUTPUT1_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT1_2), OUTPUT1_2);
    }
}
