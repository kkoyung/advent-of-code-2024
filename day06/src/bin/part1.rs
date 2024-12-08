use day06::*;
use std::fs;

fn process(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let mut state = State {
        position: find_starting_position(&map).unwrap(),
        direction: Direction::Up,
    };

    // Start moving
    loop {
        map[state.position.0][state.position.1] = 'X';

        if let Some(new_state) = step(&map, &state) {
            state = new_state;
        } else {
            break;
        }
    }

    map.iter().flatten().filter(|&&value| value == 'X').count()
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
    use day06::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
