pub const INPUT1: &str = "2333133121414131402";
pub const OUTPUT1: usize = 1928;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 2858;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Block {
    File(usize),
    Free,
}

pub fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::File(file_id) => index * file_id,
            Block::Free => 0,
        })
        .sum()
}

pub fn print_blocks(blocks: &[Block]) {
    for block in blocks {
        match block {
            Block::File(file_id) => print!("{}", file_id),
            Block::Free => print!("."),
        }
    }
    println!();
}
