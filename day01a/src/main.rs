use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    data.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|v| v.parse::<u32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    });

    left.sort();
    right.sort();

    let sum: u32 = std::iter::zip(left, right)
        .map(|(left_var, right_var)| left_var.abs_diff(right_var))
        .sum();

    println!("{}", sum);
}
