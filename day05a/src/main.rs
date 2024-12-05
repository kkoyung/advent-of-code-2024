use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let separate_line = data.lines().position(|line| line.is_empty()).unwrap();

    // Read rules
    let rules: HashMap<u8, Vec<u8>> = data
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
    let updates: Vec<Vec<u8>> = data
        .lines()
        .skip(separate_line + 1)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    // Process updates
    let sum = updates
        .iter()
        .filter(|update| is_correctly_order(update, &rules))
        .map(|update| update[update.len() / 2] as u32)
        .sum::<u32>();

    println!("{}", sum);
}

fn is_correctly_order(update: &[u8], rules: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 0..update.len() {
        if !verify_no_nums_in_list(&update[0..i], rules.get(&update[i]).unwrap_or(&vec![])) {
            return false;
        }
    }
    true
}

fn verify_no_nums_in_list(nums: &[u8], list: &[u8]) -> bool {
    list.iter()
        .filter(|num_in_list| nums.contains(num_in_list))
        .count()
        == 0
}
