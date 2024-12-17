use day17::*;
use std::fs;

fn process(input: &str) -> String {
    let (register_a, register_b, register_c, program) = parse_input(input);

    run(register_a, register_b, register_c, &program)
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(",")
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
    use day17::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert!(process(INPUT1) == OUTPUT1);
    }
}
