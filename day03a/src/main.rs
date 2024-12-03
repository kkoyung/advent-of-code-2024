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
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut state = State::Start;
    let mut sum: u32 = 0;
    let mut left_num: u32 = 0;
    let mut right_num: u32 = 0;

    for c in data.chars() {
        match state {
            State::Start => {
                if c == 'm' {
                    state = State::M;
                    continue;
                }
            },
            State::M => {
                if c == 'u' {
                    state = State::U;
                    continue;
                }
            },
            State::U => {
                if c == 'l' {
                    state = State::L;
                    continue;
                }
            },
            State::L => {
                if c == '(' {
                    state = State::LeftBracket;
                    continue;
                }
            },
            State::LeftBracket => {
                if c.is_ascii_digit() {
                    left_num = c.to_digit(10).unwrap();
                    state = State::LeftNumber;
                    continue;
                }
            },
            State::LeftNumber => {
                if c.is_ascii_digit() {
                    left_num = left_num * 10 + c.to_digit(10).unwrap();
                    continue;
                }
                else if c == ',' {
                    state = State::Comma;
                    continue;
                }
            },
            State::Comma => {
                if c.is_ascii_digit() {
                    right_num = c.to_digit(10).unwrap();
                    state = State::RightNumber;
                    continue;
                }
            },
            State::RightNumber => {
                if c.is_ascii_digit() {
                    right_num = right_num * 10 + c.to_digit(10).unwrap();
                    continue;
                }
                else if c == ')' {
                    sum += left_num * right_num;
                    state = State::Start;
                    continue;
                }
            },
            // State::RightBracket => {},
        }
        state = State::Start;
    }

    println!("{}", sum);
}
