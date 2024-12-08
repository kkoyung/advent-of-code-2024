// use day03::*;
use std::fs;

enum State {
    Start,
    M,
    U,
    L,
    LeftBracket,
    LeftNumber,
    Comma,
    RightNumber,
    // RightBracket,
    D,
    O,
    N,
    Apostrophe,
    T,
}

enum Instruction {
    Mul,
    Do,
    Dont,
}

fn process(input: &str) -> usize {
    let mut state = State::Start;
    let mut sum: u32 = 0;
    let mut left_num: u32 = 0;
    let mut right_num: u32 = 0;
    let mut instruction = Instruction::Mul;
    let mut is_enabled = true;

    for c in input.chars() {
        match state {
            State::Start => {
                if c == 'm' {
                    state = State::M;
                    continue;
                } else if c == 'd' {
                    state = State::D;
                    continue;
                }
            }
            State::M => {
                if c == 'u' {
                    state = State::U;
                    continue;
                }
            }
            State::U => {
                if c == 'l' {
                    state = State::L;
                    continue;
                }
            }
            State::L => {
                if c == '(' {
                    instruction = Instruction::Mul;
                    state = State::LeftBracket;
                    continue;
                }
            }
            State::LeftBracket => match instruction {
                Instruction::Mul => {
                    if c.is_ascii_digit() {
                        left_num = c.to_digit(10).unwrap();
                        state = State::LeftNumber;
                        continue;
                    }
                }
                Instruction::Do => {
                    if c == ')' {
                        is_enabled = true;
                        state = State::Start;
                        continue;
                    }
                }
                Instruction::Dont => {
                    if c == ')' {
                        is_enabled = false;
                        state = State::Start;
                        continue;
                    }
                }
            },
            State::LeftNumber => {
                if c.is_ascii_digit() {
                    left_num = left_num * 10 + c.to_digit(10).unwrap();
                    continue;
                } else if c == ',' {
                    state = State::Comma;
                    continue;
                }
            }
            State::Comma => {
                if c.is_ascii_digit() {
                    right_num = c.to_digit(10).unwrap();
                    state = State::RightNumber;
                    continue;
                }
            }
            State::RightNumber => {
                if c.is_ascii_digit() {
                    right_num = right_num * 10 + c.to_digit(10).unwrap();
                    continue;
                } else if c == ')' {
                    if is_enabled {
                        sum += left_num * right_num;
                    }
                    state = State::Start;
                    continue;
                }
            }
            // State::RightBracket => {},
            State::D => {
                if c == 'o' {
                    state = State::O;
                    continue;
                }
            }
            State::O => {
                if c == '(' {
                    instruction = Instruction::Do;
                    state = State::LeftBracket;
                    continue;
                } else if c == 'n' {
                    state = State::N;
                    continue;
                }
            }
            State::N => {
                if c == '\'' {
                    state = State::Apostrophe;
                    continue;
                }
            }
            State::Apostrophe => {
                if c == 't' {
                    state = State::T;
                    continue;
                }
            }
            State::T => {
                if c == '(' {
                    instruction = Instruction::Dont;
                    state = State::LeftBracket;
                    continue;
                }
            }
        }
        state = State::Start;
    }

    sum as usize
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
    use day03::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
