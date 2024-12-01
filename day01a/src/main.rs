use std::fs;

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

    left.sort();
    right.sort();

    let sum: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_var, right_var)| {
            if left_var > right_var {
                left_var - right_var
            } else {
                right_var - left_var
            }
        })
        .sum();

    println!("{}", sum);
}
