use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub const INPUT1_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
pub const OUTPUT1_1: usize = 4;

pub const INPUT1_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
pub const OUTPUT1_2: usize = 2024;

pub const INPUT2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
pub const OUTPUT2: &str = "";   // dummy test case

#[derive(PartialEq, Eq, Clone)]
pub enum Logic {
    And,
    Or,
    Xor,
}

impl Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Logic::And => write!(f, "AND"),
            Logic::Or => write!(f, "OR "),
            Logic::Xor => write!(f, "XOR"),
        }
    }
}

pub type Wire = String;
#[derive(PartialEq, Eq, Clone)]
pub struct Gate {
    pub input_1: Wire,
    pub input_2: Wire,
    pub logic: Logic,
    pub output: Wire,
}
pub type Assignment = HashMap<Wire, bool>;
pub type GateSequence = Vec<Gate>;

pub fn parse_input(input: &str) -> (Vec<Wire>, Vec<Gate>, Assignment) {
    let mut parts = input.split("\n\n");
    let init_assignment: Assignment = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut entries = line.split(": ");
            (
                Wire::from(entries.next().unwrap()),
                entries.next().unwrap() == "1",
            )
        })
        .collect();
    let gates: Vec<Gate> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut entries = line.split_whitespace();
            Gate {
                input_1: Wire::from(entries.next().unwrap()),
                logic: match entries.next().unwrap() {
                    "AND" => Logic::And,
                    "OR" => Logic::Or,
                    "XOR" => Logic::Xor,
                    _ => panic!("Unknown gate logic"),
                },
                input_2: Wire::from(entries.next().unwrap()),
                output: Wire::from(entries.nth(1).unwrap()),
            }
        })
        .collect();
    let mut wires: Vec<Wire> = gates
        .iter()
        .flat_map(|gate| {
            [
                gate.input_1.clone(),
                gate.input_2.clone(),
                gate.output.clone(),
            ]
        })
        .collect::<HashSet<Wire>>()
        .into_iter()
        .collect();
    wires.sort();

    (wires, gates, init_assignment)
}

pub fn run_gates(gates: &[Gate], init_assignment: &Assignment) -> (Assignment, GateSequence) {
    let mut assignment = init_assignment.clone();
    let mut gate_sequence: GateSequence = GateSequence::new();

    let mut gate_count = 1;
    while gate_count > 0 {
        gate_count = gates
            .iter()
            .map(|gate| {
                if !assignment.contains_key(&gate.output) {
                    if let Some(input_1) = assignment.get(&gate.input_1) {
                        if let Some(input_2) = assignment.get(&gate.input_2) {
                            match gate.logic {
                                Logic::And => {
                                    assignment.insert(gate.output.clone(), input_1 & input_2);
                                }
                                Logic::Or => {
                                    assignment.insert(gate.output.clone(), input_1 | input_2);
                                }
                                Logic::Xor => {
                                    assignment.insert(gate.output.clone(), input_1 ^ input_2);
                                }
                            }
                            gate_sequence.push(gate.clone());
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum();
    }

    (assignment, gate_sequence)
}

pub fn wires_to_number(assignment: &Assignment, starts_with: &str) -> usize {
    let mut related_wires: Vec<(Wire, bool)> = assignment
        .iter()
        .filter(|(name, _state)| name.starts_with(starts_with))
        .map(|(name, state)| (name.clone(), *state))
        .collect();
    related_wires.sort_by_key(|(name, _state)| name.clone());
    related_wires
        .iter()
        .map(|(_name, state)| if *state { 1 } else { 0 })
        .enumerate()
        .fold(0, |sum, (i, num)| sum + (num * 2usize.pow(i as u32)))
}
