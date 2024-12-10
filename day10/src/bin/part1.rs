use day10::*;
use std::{collections::HashMap, fs};

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Map>();

    let trailtails = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, level)| (i, j, level)))
        .filter_map(|(i, j, level)| if *level == 9 { Some((i, j)) } else { None })
        .collect::<Vec<Position>>();

    trailtails
        .iter()
        .flat_map(|position| {
            let mut tails = vec![*position];
            for _ in 0..9 {
                tails = tails
                    .iter()
                    .flat_map(|position| move_down(&map, position))
                    .collect::<Vec<Position>>();
            }
            tails.sort();
            tails.dedup();
            tails
        })
        .fold(HashMap::new(), |mut mapper, position| {
            mapper.insert(position, mapper.get(&position).unwrap_or(&0) + 1);
            mapper
        })
        .values()
        .sum()
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
    use day10::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
