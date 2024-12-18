use day18::*;
use std::{collections::BTreeSet, fs};

fn process(input: &str, max_coordinate: usize, number_of_bytes: usize) -> usize {
    // Parse input
    let bytes = input
        .lines()
        .map(|line| {
            let mut numbers = line.split(",").map(|text| text.parse::<usize>().unwrap());
            Position {
                x: numbers.next().unwrap(),
                y: numbers.next().unwrap(),
            }
        })
        .collect::<Vec<Position>>();

    // Initialize
    let mut map: Map = vec![vec![Space::Free; max_coordinate + 1]; max_coordinate + 1];
    map[0][0] = Space::Step(0);
    let mut step_table: Vec<BTreeSet<Position>> =
        vec![BTreeSet::from([Position { x: 0, y: 0 }])];

    // Drop some bytes
    bytes
        .iter()
        .take(number_of_bytes)
        .for_each(|byte| map[byte.x][byte.y] = Space::Corrupted);
    run(max_coordinate, &mut map, &mut step_table);

    step_table.len() - 1
}

// =====================================================================

fn main() {
    println!("Part 1");

    let input = fs::read_to_string("input").expect("Unable to read file");
    let max_coordinate = 70;
    let number_of_bytes = 1024;
    println!(
        "{}",
        process(input.as_str(), max_coordinate, number_of_bytes)
    );
}

#[cfg(test)]
mod tests {
    use super::process;
    use day18::{INPUT1, MAX_COORDINATE1, NUMBER_OF_BYTES1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1, MAX_COORDINATE1, NUMBER_OF_BYTES1), OUTPUT1);
    }
}
