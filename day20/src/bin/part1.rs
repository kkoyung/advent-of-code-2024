use day20::*;
use std::fs;

fn process(input: &str) -> usize {
    let (map, track) = parse_input(input);
    let height = map.len();
    let width = map[0].len();

    track
        .iter()
        .flat_map(|start_position| {
            let mut cheats: Vec<(Position, Position)> = Vec::new();
            if start_position.0 >= 2 {
                let end_position = (start_position.0 - 2, start_position.1);
                cheats.push((*start_position, end_position));
            }
            if start_position.0 >= 1 && start_position.1 >= 1 {
                let end_position = (start_position.0 - 1, start_position.1 - 1);
                cheats.push((*start_position, end_position));
            }
            if start_position.1 >= 2 {
                let end_position = (start_position.0, start_position.1 - 2);
                cheats.push((*start_position, end_position));
            }
            if start_position.0 < height - 1 && start_position.1 >= 1 {
                let end_position = (start_position.0 + 1, start_position.1 - 1);
                cheats.push((*start_position, end_position));
            }
            if start_position.0 < height - 2 {
                let end_position = (start_position.0 + 2, start_position.1);
                cheats.push((*start_position, end_position));
            }
            if start_position.0 < height - 1 && start_position.1 < width - 1 {
                let end_position = (start_position.0 + 1, start_position.1 + 1);
                cheats.push((*start_position, end_position));
            }
            if start_position.1 < width - 2 {
                let end_position = (start_position.0, start_position.1 + 2);
                cheats.push((*start_position, end_position));
            }
            if start_position.0 >= 1 && start_position.1 < width - 1 {
                let end_position = (start_position.0 - 1, start_position.1 + 1);
                cheats.push((*start_position, end_position));
            }
            cheats
        })
        .filter_map(|(start_position, end_position)| {
            match (
                &map[start_position.0][start_position.1],
                &map[end_position.0][end_position.1],
            ) {
                (Tile::Picoseconds(start_picoseconds), Tile::Picoseconds(end_picoseconds)) => {
                    Some((*start_picoseconds, *end_picoseconds))
                }
                _ => None,
            }
        })
        .map(|(start_picoseconds, end_picoseconds)| {
            end_picoseconds as i32 - (start_picoseconds + 2) as i32
        })
        .filter(|&save| save >= 100)
        .count()
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
    use day20::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
