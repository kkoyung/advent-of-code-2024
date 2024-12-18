use day18::*;
use std::{collections::BTreeSet, fs};

fn process(input: &str, max_coordinate: usize, number_of_bytes: usize) -> String {
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

    for byte in bytes.iter().skip(number_of_bytes) {
        let restart_step = match map[byte.x][byte.y] {
            Space::Free => continue,
            Space::Corrupted => {
                panic!("Byte is dropped at wrong place")
            }
            Space::Step(step) => step,
        };

        // Reset steps after restart_step
        map[byte.x][byte.y] = Space::Corrupted;
        map.iter_mut().for_each(|column| {
            column.iter_mut().for_each(|space| {
                *space = match space {
                    Space::Step(step) => {
                        if *step >= restart_step {
                            Space::Free
                        } else {
                            Space::Step(*step)
                        }
                    }
                    Space::Free => Space::Free,
                    Space::Corrupted => Space::Corrupted,
                }
            });
        });
        step_table = step_table
            .into_iter()
            .take(restart_step)
            .collect::<Vec<BTreeSet<Position>>>();

        // Continue from restart_step
        if !run(max_coordinate, &mut map, &mut step_table) {
            return format!("{},{}", byte.x, byte.y);
        }
    }

    String::from("")
}

// =====================================================================

fn main() {
    println!("Part 2");

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
    use day18::{INPUT2, MAX_COORDINATE2, NUMBER_OF_BYTES2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2, MAX_COORDINATE2, NUMBER_OF_BYTES2), OUTPUT2);
    }
}
