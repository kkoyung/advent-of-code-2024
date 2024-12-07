use std::fs;
use template::*;

fn main() {
    println!("Part 2");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

fn process(input: &str) -> usize {
    // Write here
    input.len()
}

#[cfg(test)]
mod tests {
    use crate::process;
    use template::INPUT;

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT), 0);
    }
}
