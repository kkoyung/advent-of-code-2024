// use day08::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Map>();
    let height = map.len();
    let width = map[0].len();

    let antennas = find_antennas(&map);
    let sum = antennas
        .values()
        .flat_map(|positions| find_antinodes(positions, height, width))
        .collect::<HashSet<Position>>()
        .len();

    sum
}

fn find_antennas(map: &Map) -> HashMap<char, Vec<(usize, usize)>> {
    let antennas = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, value)| (i, j, value)))
        .filter(|(_i, _j, value)| **value != '.')
        .fold(
            HashMap::new(),
            |mut antennas: HashMap<char, Vec<Position>>, (i, j, value)| {
                if let Some(positions) = antennas.get_mut(value) {
                    positions.push((i, j));
                } else {
                    antennas.insert(*value, vec![(i, j)]);
                }
                antennas
            },
        );

    antennas
}
fn find_antinodes(positions: &[Position], height: usize, width: usize) -> HashSet<Position> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            antinodes.extend(find_antinodes_of_pairs(
                &positions[i],
                &positions[j],
                height,
                width,
            ));
        }
    }

    antinodes
}

fn find_antinodes_of_pairs(
    first: &Position,
    second: &Position,
    height: usize,
    width: usize,
) -> HashSet<Position> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    let mut first = (first.0 as i64, first.1 as i64);
    let mut second = (second.0 as i64, second.1 as i64);
    let diff = (second.0 - first.0, second.1 - first.1);

    second.0 += diff.0;
    second.1 += diff.1;
    if second.0 >= 0 && second.0 < height as i64 && second.1 >= 0 && second.1 < width as i64 {
        antinodes.insert((second.0 as usize, second.1 as usize));
    }

    first.0 -= diff.0;
    first.1 -= diff.1;
    if first.0 >= 0 && first.0 < height as i64 && first.1 >= 0 && first.1 < width as i64 {
        antinodes.insert((first.0 as usize, first.1 as usize));
    }

    antinodes
}

fn main() {
    println!("Part 1");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

#[cfg(test)]
mod tests {
    use crate::process;
    use day08::INPUT;

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT), 14);
    }
}
