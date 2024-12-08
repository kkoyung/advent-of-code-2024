use day04::*;
use std::fs;

const TARGET: [char; 3] = ['M', 'A', 'S'];

fn count(states: &[Vec<u8>]) -> usize {
    let height = states.len();
    let width = states[0].len();

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if states[i][j] & NORTH_WEST > 0 {
                if (i <= height - 3) && (states[i + 2][j] & SOUTH_WEST > 0) {
                    sum += 1;
                }
                if (j <= width - 3) && (states[i][j + 2] & NORTH_EAST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & NORTH_EAST > 0 {
                if (i <= height - 3) && (states[i + 2][j] & SOUTH_EAST > 0) {
                    sum += 1;
                }
                if (j >= 2) && (states[i][j - 2] & NORTH_WEST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & SOUTH_EAST > 0 {
                if (i >= 2) && (states[i - 2][j] & NORTH_EAST > 0) {
                    sum += 1;
                }
                if (j >= 2) && (states[i][j - 2] & SOUTH_WEST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & SOUTH_WEST > 0 {
                if (i >= 2) && (states[i - 2][j] & NORTH_WEST > 0) {
                    sum += 1;
                }
                if (j <= width - 3) && (states[i][j + 2] & SOUTH_EAST > 0) {
                    sum += 1;
                }
            }
        }
    }

    sum / 2
}

fn process(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut states: Vec<Vec<u8>> = first(&map, TARGET[0]);
    for target in &TARGET[1..] {
        states = step(&map, &states, *target);
    }

    count(&states)
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
    use day04::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
