use day06::*;
use std::{collections::HashSet, fs};

fn is_adding_at_the_front_forms_loop(
    map: &Map,
    state: &State,
    trace: &Trace,
) -> Option<(usize, usize)> {
    let mut map = map.clone();
    let mut state = *state;
    let mut trace = trace.clone();

    if let Some((front_position, front_value)) = front(&map, &state) {
        if front_value == '.'
            && !trace
                .iter()
                .map(|state| state.position)
                .collect::<HashSet<(usize, usize)>>()
                .contains(&front_position)
        {
            map[front_position.0][front_position.1] = '#';
            state.direction = turn_right(&state.direction);

            loop {
                if trace.contains(&state) {
                    return Some(front_position);
                }

                trace.insert(state);
                if let Some(new_state) = step(&map, &state) {
                    state = new_state;
                } else {
                    break;
                }
            }
        }
    }
    None
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let mut state = State {
        position: find_starting_position(&map).unwrap(),
        direction: Direction::Up,
    };

    let mut trace: Trace = Trace::new();
    let mut new_obstruction: HashSet<(usize, usize)> = HashSet::new();

    // Start moving
    loop {
        if let Some(obstruction) = is_adding_at_the_front_forms_loop(&map, &state, &trace) {
            new_obstruction.insert(obstruction);
        }

        trace.insert(state);
        if let Some(new_state) = step(&map, &state) {
            state = new_state;
        } else {
            break;
        }
    }

    new_obstruction.len()
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
    use day06::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
