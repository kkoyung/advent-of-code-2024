use day09::*;
use std::fs;

fn next_left_most_free(blocks: &[Block], mut index: usize) -> Option<usize> {
    while !matches!(blocks[index], Block::Free) {
        if index >= blocks.len() - 1 {
            return None;
        }
        index += 1;
    }
    Some(index)
}

fn next_right_most_file(blocks: &[Block], mut index: usize) -> Option<usize> {
    while !matches!(blocks[index], Block::File(_)) {
        if index == 0 {
            return None;
        }
        index -= 1;
    }
    Some(index)
}

fn process(input: &str) -> usize {
    let mut blocks = input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(index, digit)| {
            if index % 2 == 0 {
                vec![Block::File(index / 2); digit]
            } else {
                vec![Block::Free; digit]
            }
        })
        .collect::<Vec<Block>>();

    let mut left_most_free = next_left_most_free(&blocks, 0).unwrap();
    let mut right_most_file = next_right_most_file(&blocks, blocks.len() - 1).unwrap();

    while left_most_free < right_most_file {
        blocks.swap(left_most_free, right_most_file);

        left_most_free = next_left_most_free(&blocks, left_most_free).unwrap_or(blocks.len());
        right_most_file = next_right_most_file(&blocks, right_most_file).unwrap_or(0);
    }

    checksum(&blocks)
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
    use day09::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
