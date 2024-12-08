use day05::*;
use std::{collections::HashMap, fs};

fn process(input: &str) -> usize {
    let separate_line = input.lines().position(|line| line.is_empty()).unwrap();

    // Read rules
    let rules: HashMap<u8, Vec<u8>> = input
        .lines()
        .take(separate_line)
        .map(|line| line.split("|").map(|text| text.parse::<u8>().unwrap()))
        .fold(HashMap::new(), |mut rules, mut pair| {
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            rules
                .entry(left)
                .and_modify(|list: &mut Vec<u8>| list.push(right))
                .or_insert(vec![right]);
            rules
        });

    // Read updates
    let updates: Vec<Vec<u8>> = input
        .lines()
        .skip(separate_line + 1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    // Process updates
    updates
        .iter()
        .filter(|update| is_correctly_order(update, &rules))
        .map(|update| update[update.len() / 2] as usize)
        .sum::<usize>()
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
    use day05::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
