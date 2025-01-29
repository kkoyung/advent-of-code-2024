use day21::*;
use std::fs;

fn process(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let fourth_keypad = Box::new(HumanKeypad::new(Layout::new_directional()));
    let third_keypad = Box::new(RobotKeypad::new(Layout::new_directional(), fourth_keypad));
    let second_keypad = Box::new(RobotKeypad::new(Layout::new_directional(), third_keypad));
    let mut first_keypad = Box::new(RobotKeypad::new(Layout::new_numeric(), second_keypad));

    codes
        .iter()
        .map(|code| (first_keypad.cost_of_sequence(code), code_to_number(code)))
        .map(|(length, numeric)| length * numeric)
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
    use day21::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
