use day09::*;
use std::fs;

#[derive(Clone, Copy)]
struct Chuck {
    block: Block,
    size: usize,
}

fn chuck_to_blocks(chucks: &[Chuck]) -> Vec<Block> {
    chucks
        .iter()
        .flat_map(|chuck| vec![chuck.block; chuck.size])
        .collect::<Vec<Block>>()
}

fn process(input: &str) -> usize {
    let mut chucks = input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(index, digit)| {
            if index % 2 == 0 {
                Chuck {
                    block: Block::File(index / 2),
                    size: digit,
                }
            } else {
                Chuck {
                    block: Block::Free,
                    size: digit,
                }
            }
        })
        .collect::<Vec<Chuck>>();

    let mut file_id = (chucks.len() - 1) / 2;
    loop {
        let file_position = chucks
            .iter()
            .rposition(|chuck| chuck.block == Block::File(file_id))
            .unwrap();
        let file_size = chucks[file_position].size;
        let space_position = chucks
            .iter()
            .position(|chuck| chuck.block == Block::Free && chuck.size >= file_size);

        if let Some(space_position) = space_position {
            if file_position > space_position {
                // move out file
                let file_chuck = chucks[file_position];
                chucks[file_position].block = Block::Free;
                // write to space
                chucks[space_position].size -= file_chuck.size;
                chucks.insert(space_position, file_chuck);
            }
        }

        if file_id == 0 {
            break;
        } else {
            file_id -= 1;
        }
    }

    checksum(&chuck_to_blocks(&chucks))
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
    use day09::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
