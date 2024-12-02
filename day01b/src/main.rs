use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    data.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|v| v.parse::<u32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    });

    let mut hash: HashMap<u32, u32> = HashMap::new();
    right.iter().for_each(|&v| {
        hash.insert(v, *hash.get(&v).unwrap_or(&0) + 1);
    });

    let sum = left.iter().map(|&v| {
        v * (*hash.get(&v).unwrap_or(&0))
    }).sum::<u32>();

    println!("{}", sum);
}
