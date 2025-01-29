use day21::*;
use std::fs;

fn process(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let human_keypad = Box::new(HumanKeypad::new(Layout::new_directional()));
    let mut middle_keypad = Box::new(RobotKeypad::new(Layout::new_directional(), human_keypad));
    for _ in 1..25 {
        middle_keypad = Box::new(RobotKeypad::new(Layout::new_directional(), middle_keypad));
    }
    let mut first_keypad = RobotKeypad::new(Layout::new_numeric(), middle_keypad);

    codes
        .iter()
        .map(|code| (first_keypad.cost_of_sequence(code), code_to_number(code)))
        .map(|(length, numeric)| length * numeric)
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
    use day21::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
