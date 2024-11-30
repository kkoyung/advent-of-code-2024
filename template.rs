use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let mut sum: u32 = 0;

    for line in data.lines() {
    }
    println!("{}", sum);
}
