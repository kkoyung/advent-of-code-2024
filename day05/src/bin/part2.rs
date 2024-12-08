use day05::*;
use std::{collections::HashMap, fs};

fn re_order(update: &[u8], rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut new_update = Vec::from(update);
    let mut i = 0;
    while i < new_update.len() {
        let empty_rule = vec![];
        let rule = rules.get(&new_update[i]).unwrap_or(&empty_rule);
        for j in 0..i {
            if rule.contains(&new_update[j]) {
                new_update.push(new_update[j]);
                new_update[j] = 0;
            }
        }

        i += 1;
    }

    new_update
        .into_iter()
        .filter(|num| *num != 0)
        .collect::<Vec<u8>>()
}

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
        .filter(|update| !is_correctly_order(update, &rules))
        .map(|update| re_order(update, &rules))
        .map(|update| update[update.len() / 2] as u32)
        .sum::<u32>() as usize
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
    use day05::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
