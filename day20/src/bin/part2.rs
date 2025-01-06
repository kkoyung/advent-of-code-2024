use day20::*;
use std::fs;

fn process(input: &str) -> usize {
    let (map, track) = parse_input(input);
    let height = map.len();
    let width = map[0].len();

    track
        .iter()
        .flat_map(|start_position| {
            (0..height)
                .flat_map(|i| (0..width).map(move |j| (i, j)))
                .filter(|(i, j)| {
                    start_position.0.abs_diff(*i) + start_position.1.abs_diff(*j) <= 20
                })
                .map(|end_position| (*start_position, end_position))
                .collect::<Vec<(Position, Position)>>()
        })
        .filter_map(|(start_position, end_position)| {
            match (
                &map[start_position.0][start_position.1],
                &map[end_position.0][end_position.1],
            ) {
                (Tile::Picoseconds(start_picoseconds), Tile::Picoseconds(end_picoseconds)) => {
                    Some((
                        start_position,
                        end_position,
                        *start_picoseconds,
                        *end_picoseconds,
                    ))
                }
                _ => None,
            }
        })
        .map(
            |(start_position, end_position, start_picoseconds, end_picoseconds)| {
                end_picoseconds as i32
                    - (start_picoseconds
                        + start_position.0.abs_diff(end_position.0)
                        + start_position.1.abs_diff(end_position.1)) as i32
            },
        )
        .filter(|&save| save >= 100)
        .count()
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
    use day20::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
