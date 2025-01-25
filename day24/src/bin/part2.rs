use day24::*;
use itertools::Itertools;
use std::{collections::HashMap, fs};

fn process(input: &str) -> String {
    let (_wires, gates, init_assignment) = parse_input(input);
    let mut problematic_gates: Vec<&Gate> = Vec::new();

    // ================================================================

    // For any XOR gate with x input and y input,
    // its output should be an input of an XOR gate and an input of an AND gate.
    // The XOR gate and AND gate should also share the same two inputs.
    // Unless, the x input is "x00" and the y input is "y00".

    let x_xor_y: Vec<&Gate> = gates
        .iter()
        .filter(|gate| {
            (gate.input_1.starts_with("x") && gate.input_2.starts_with("y"))
                || (gate.input_1.starts_with("y") || gate.input_2.starts_with("x"))
        })
        .filter(|gate| gate.logic == Logic::Xor)
        .collect();
    x_xor_y
        .iter()
        .filter(|gate| {
            ***gate
                != Gate {
                    input_1: String::from("x00"),
                    input_2: String::from("y00"),
                    logic: Logic::Xor,
                    output: String::from("z00"),
                }
                && ***gate
                    != Gate {
                        input_1: String::from("y00"),
                        input_2: String::from("x00"),
                        logic: Logic::Xor,
                        output: String::from("z00"),
                    }
        })
        .filter(|gate| {
            let and_children = gates
                .iter()
                .filter(|other_gate| {
                    (other_gate.input_1 == gate.output || other_gate.input_2 == gate.output)
                        && other_gate.logic == Logic::And
                })
                .collect::<Vec<&Gate>>();
            let and_sibling: Option<String> = match and_children.first() {
                Some(&and_child) => {
                    if and_child.input_1 == gate.output {
                        Some(and_child.input_2.clone())
                    } else {
                        Some(and_child.input_1.clone())
                    }
                }
                None => None,
            };
            let xor_children = gates
                .iter()
                .filter(|other_gate| {
                    (gate.output == other_gate.input_1 || gate.output == other_gate.input_2)
                        && other_gate.logic == Logic::Xor
                })
                .collect::<Vec<&Gate>>();
            let xor_sibling = match and_children.first() {
                Some(&and_child) => {
                    if and_child.input_1 == gate.output {
                        Some(and_child.input_2.clone())
                    } else {
                        Some(and_child.input_1.clone())
                    }
                }
                None => None,
            };
            !(and_children.len() == 1
                && xor_children.len() == 1
                && and_sibling.is_some()
                && xor_sibling.is_some()
                && and_sibling == xor_sibling)
        })
        .for_each(|gate| {
            print_gate(gate);
            problematic_gates.push(gate);
        });

    // ================================================================

    // For any AND gate with x input and y input,
    // its output should be an input of an OR gate.

    let x_and_y: Vec<&Gate> = gates
        .iter()
        .filter(|gate| {
            (gate.input_1.starts_with("x") && gate.input_2.starts_with("y"))
                || (gate.input_1.starts_with("y") || gate.input_2.starts_with("x"))
        })
        .filter(|gate| gate.logic == Logic::And)
        .collect();
    x_and_y
        .iter()
        .filter(|gate| {
            !((gate.input_1 == "x00" && gate.input_2 == "y00")
                || (gate.input_1 == "y00" && gate.input_2 == "x00"))
        })
        .filter(|gate| {
            let or_children = gates
                .iter()
                .filter(|other_gate| {
                    (other_gate.input_1 == gate.output || other_gate.input_2 == gate.output)
                        && other_gate.logic == Logic::Or
                })
                .collect::<Vec<&Gate>>();
            or_children.len() != 1
        })
        .for_each(|gate| {
            print_gate(gate);
            problematic_gates.push(gate);
        });

    // ================================================================

    // For any XOR gate with non-x input and non-y input,
    // its output should be a z-wire.

    let a_xor_b: Vec<&Gate> = gates
        .iter()
        .filter(|gate| {
            !gate.input_1.starts_with("x")
                && !gate.input_1.starts_with("y")
                && !gate.input_2.starts_with("x")
                && !gate.input_2.starts_with("y")
        })
        .filter(|gate| gate.logic == Logic::Xor)
        .collect();
    a_xor_b
        .iter()
        .filter(|gate| !gate.output.starts_with("z"))
        .for_each(|gate| {
            print_gate(gate);
            problematic_gates.push(gate);
        });

    // ================================================================

    // For any AND gate with non-x input and non-y input,
    // its output should be an input of an OR gate.

    let a_and_b: Vec<&Gate> = gates
        .iter()
        .filter(|gate| {
            !gate.input_1.starts_with("x")
                && !gate.input_1.starts_with("y")
                && !gate.input_2.starts_with("x")
                && !gate.input_2.starts_with("y")
        })
        .filter(|gate| gate.logic == Logic::And)
        .collect();
    a_and_b
        .iter()
        .filter(|gate| {
            let or_children = gates
                .iter()
                .filter(|other_gate| {
                    (other_gate.input_1 == gate.output || other_gate.input_2 == gate.output)
                        && other_gate.logic == Logic::Or
                })
                .collect::<Vec<&Gate>>();
            or_children.len() != 1
        })
        .for_each(|gate| {
            print_gate(gate);
            problematic_gates.push(gate);
        });

    // ================================================================

    // For any OR gate,
    // its output should be not be a z-wire (since its output is the carry).
    // Unless its output is "z45".

    let a_or_b: Vec<&Gate> = gates
        .iter()
        .filter(|gate| gate.logic == Logic::Or)
        .collect();
    a_or_b
        .iter()
        .filter(|gate| gate.output.starts_with("z"))
        .filter(|gate| gate.output != "z45")
        .for_each(|gate| {
            print_gate(gate);
            problematic_gates.push(gate);
        });

    // ================================================================

    // We got 8 problematic gates here.
    // We try different swaps among the outputs of those gates to see whether
    // we can fix the device.

    if problematic_gates.len() != 8 {
        return String::from("");
    }

    let problematic_outputs: Vec<Wire> = problematic_gates
        .iter()
        .map(|gate| gate.output.clone())
        .collect();
    let all_possible_swap = possible_swap(&problematic_outputs);

    if let Some(swap) = all_possible_swap
        .iter()
        .find(|swap| swap_and_test(&init_assignment, gates.clone(), swap))
    {
        println!("Find a swap that fixes the device");
        let mut wires = swap.keys().collect::<Vec<&Wire>>();
        wires.sort();
        wires.into_iter().join(",")
    } else {
        String::from("")
    }
}

fn print_gate(gate: &Gate) {
    println!(
        "{} {} {} -> {}",
        gate.input_1, gate.logic, gate.input_2, gate.output
    );
}

fn possible_swap(outputs: &[Wire]) -> Vec<HashMap<Wire, Wire>> {
    // We should get 8 problematic outputs.
    if outputs.len() != 8 {
        panic!("We should get 8 problematic outputs.");
    }

    let mut permutations = outputs
        .iter()
        .permutations(8)
        .map(|permutation| permutation.into_iter().tuples())
        .map(|partition| {
            // standard expression
            let mut partition = partition
                .into_iter()
                .map(|pair: (&Wire, &Wire)| {
                    if pair.0 < pair.1 {
                        (pair.0, pair.1)
                    } else {
                        (pair.1, pair.0)
                    }
                })
                .collect::<Vec<(&Wire, &Wire)>>();
            partition.sort();
            partition
        })
        .collect::<Vec<Vec<(&Wire, &Wire)>>>();
    permutations.sort();
    permutations.dedup();

    permutations
        .iter()
        .map(|permutation| {
            permutation.iter().fold(HashMap::new(), |mut map, pair| {
                map.insert(pair.0.clone(), pair.1.clone());
                map.insert(pair.1.clone(), pair.0.clone());
                map
            })
        })
        .collect::<Vec<HashMap<Wire, Wire>>>()
}

fn swap_and_test(
    init_assignment: &Assignment,
    mut gates: Vec<Gate>,
    swap: &HashMap<Wire, Wire>,
) -> bool {
    gates.iter_mut().for_each(|gate| {
        if swap.contains_key(&gate.output) {
            gate.output = swap.get(&gate.output).unwrap().to_string();
        }
    });
    let (assignment, _gate_sequence) = run_gates(&gates, init_assignment);
    let x = wires_to_number(&assignment, "x");
    let y = wires_to_number(&assignment, "y");
    let z = wires_to_number(&assignment, "z");

    x + y == z
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
    use day24::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
