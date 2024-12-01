use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    data.lines()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .for_each(|v: Vec<u32>| {
            left.push(v[0]);
            right.push(v[1]);
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
