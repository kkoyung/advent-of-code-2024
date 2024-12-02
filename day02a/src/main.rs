use std::fs;

fn is_safe(nums: &[i32]) -> bool {
    let is_all_increasing_gradually = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .all(|diff| (1..=3).contains(&diff));
    let is_all_decreasing_gradually = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .all(|diff| (-3..=-1).contains(&diff));

    is_all_increasing_gradually || is_all_decreasing_gradually
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let sum = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_safe(&nums[..]))
        .count();

    println!("{}", sum);
}
