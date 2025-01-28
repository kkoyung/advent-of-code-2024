use std::collections::BTreeMap;
use itertools::Itertools;

pub const INPUT1: &str = "029A
980A
179A
456A
379A";
pub const OUTPUT1: usize = 126384;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 0;

#[derive(Clone, Copy)]
struct Position(usize, usize);

pub struct Layout {
    layout: Vec<Vec<Option<char>>>,
    positions: BTreeMap<char, Position>,
}

impl Layout {
    pub fn new_numeric() -> Layout {
        let layout = vec![
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ];
        let positions: BTreeMap<char, Position> = [
            ('7', Position(0, 0)),
            ('8', Position(0, 1)),
            ('9', Position(0, 2)),
            ('4', Position(1, 0)),
            ('5', Position(1, 1)),
            ('6', Position(1, 2)),
            ('1', Position(2, 0)),
            ('2', Position(2, 1)),
            ('3', Position(2, 2)),
            ('0', Position(3, 1)),
            ('A', Position(3, 2)),
        ]
        .into_iter()
        .collect();
        Layout { layout, positions }
    }

    pub fn new_directional() -> Layout {
        let layout = vec![
            vec![None, Some('^'), Some('A')],
            vec![Some('<'), Some('v'), Some('>')],
        ];
        let positions: BTreeMap<char, Position> = [
            ('^', Position(0, 1)),
            ('A', Position(0, 2)),
            ('<', Position(1, 0)),
            ('v', Position(1, 1)),
            ('>', Position(1, 2)),
        ]
        .into_iter()
        .collect();
        Layout { layout, positions }
    }

    fn possible_move_sequences(&self, arm: char, key: char) -> Vec<Vec<char>> {
        let arm_position = self.positions.get(&arm).unwrap().to_owned();
        let key_position = self.positions.get(&key).unwrap().to_owned();
        let mut basic_sequence: Vec<char> = Vec::new();
        if arm_position.0 < key_position.0 {
            basic_sequence.extend(vec!['v'; arm_position.0.abs_diff(key_position.0)]);
        } else {
            basic_sequence.extend(vec!['^'; arm_position.0.abs_diff(key_position.0)]);
        }
        if arm_position.1 < key_position.1 {
            basic_sequence.extend(vec!['>'; arm_position.1.abs_diff(key_position.1)]);
        } else {
            basic_sequence.extend(vec!['<'; arm_position.1.abs_diff(key_position.1)]);
        }

        basic_sequence
            .iter()
            .permutations(basic_sequence.len())
            .unique()
            .map(|seq| seq.into_iter().cloned().collect::<Vec<char>>())
            .filter(|seq| {
                seq.iter()
                    .try_fold(arm_position, |position, direction| {
                        let new_position = match direction {
                            '^' => Position(position.0 - 1, position.1),
                            'v' => Position(position.0 + 1, position.1),
                            '<' => Position(position.0, position.1 - 1),
                            '>' => Position(position.0, position.1 + 1),
                            _ => panic!("Unknown direction"),
                        };
                        if self.layout[new_position.0][new_position.1].is_some() {
                            Some(new_position)
                        } else {
                            None
                        }
                    })
                    .is_some()
            })
            .map(|mut sequence| {
                sequence.push('A');
                sequence
            })
            .collect::<Vec<Vec<char>>>()
    }
}

pub trait Keypad {
    fn cost_of_key_press(&self, arm: char, key: char) -> usize;
    fn cost_of_sequence(&self, sequence: &[char]) -> usize;
}

pub struct HumanKeypad {
    _layout: Layout,
}

impl HumanKeypad {
    pub fn new(layout: Layout) -> HumanKeypad {
        HumanKeypad { _layout: layout }
    }
}

impl Keypad for HumanKeypad {
    fn cost_of_key_press(&self, _arm: char, _key: char) -> usize {
        1
    }

    fn cost_of_sequence(&self, sequence: &[char]) -> usize {
        sequence.len()
    }
}

pub struct RobotKeypad<T: Keypad> {
    layout: Layout,
    upstream_keypad: T,
}

impl<T: Keypad> RobotKeypad<T> {
    pub fn new(layout: Layout, upstream_keypad: T) -> RobotKeypad<T> {
        RobotKeypad {
            layout,
            upstream_keypad,
        }
    }
}

impl<T: Keypad> Keypad for RobotKeypad<T> {
    fn cost_of_key_press(&self, arm: char, key: char) -> usize {
        self.layout
            .possible_move_sequences(arm, key)
            .iter()
            .map(|sequence| self.upstream_keypad.cost_of_sequence(sequence))
            .min()
            .unwrap()
    }

    fn cost_of_sequence(&self, sequence: &[char]) -> usize {
        let mut padded_sequence = vec!['A'];
        padded_sequence.extend(sequence);
        padded_sequence
            .windows(2)
            .map(|pair| self.cost_of_key_press(pair[0], pair[1]))
            .sum()
    }
}

pub fn code_to_number(code: &[char]) -> usize {
    code.iter()
        .fold(0, |num, character| match character.to_digit(10) {
            Some(digit) => num * 10 + (digit as usize),
            None => num,
        })
}
