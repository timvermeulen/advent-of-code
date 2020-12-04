use parser::prelude::*;

#[derive(Copy, Clone)]
enum Operation {
    SwapPositions { x: usize, y: usize },
    SwapLetters { x: u8, y: u8 },
    RotateLeft { by: usize },
    RotateRight { by: usize },
    Move { from: usize, to: usize },
    Reverse { from: usize, through: usize },
    Rotate { based_on: u8 },
}

impl Operation {
    fn reverse(self) -> ReversedOperation {
        match self {
            Self::RotateLeft { by } => ReversedOperation::Operation(Self::RotateRight { by }),
            Self::RotateRight { by } => ReversedOperation::Operation(Self::RotateLeft { by }),
            Self::Move { from, to } => {
                ReversedOperation::Operation(Self::Move { from: to, to: from })
            }
            Self::Rotate { based_on } => ReversedOperation::ReversedRotate { based_on },
            _ => ReversedOperation::Operation(self),
        }
    }

    fn operate(self, letters: &mut [u8; 8]) {
        match self {
            Self::SwapPositions { x, y } => letters.swap(x, y),
            Self::SwapLetters { x, y } => {
                let i = letters.iter().position(|&l| l == x).unwrap();
                let j = letters.iter().position(|&l| l == y).unwrap();
                letters.swap(i, j);
            }
            Self::RotateLeft { by } => letters.rotate_left(by),
            Self::RotateRight { by } => letters.rotate_right(by),
            Self::Move { from, to } => {
                let letter = letters[from];
                if from > to {
                    letters.copy_within(to..from, to + 1);
                } else {
                    letters.copy_within(from + 1..=to, from);
                }
                letters[to] = letter;
            }
            Self::Reverse { from, through } => letters[from..=through].reverse(),
            Self::Rotate { based_on } => {
                let i = letters.iter().position(|&l| l == based_on).unwrap();
                let amount = [1, 2, 3, 4, 6, 7, 0, 1][i];
                letters.rotate_right(amount);
            }
        }
    }
}

#[derive(Copy, Clone)]
enum ReversedOperation {
    Operation(Operation),
    ReversedRotate { based_on: u8 },
}

impl ReversedOperation {
    fn operate(self, letters: &mut [u8; 8]) {
        match self {
            Self::Operation(op) => op.operate(letters),
            Self::ReversedRotate { based_on } => {
                let i = letters.iter().position(|&l| l == based_on).unwrap();
                let amount = [7, 7, 2, 6, 1, 5, 0, 4][i];
                letters.rotate_right(amount);
            }
        }
    }
}

fn parse(mut input: &str) -> Vec<Operation> {
    let position = parser::usize();
    let letter = parser::any().map(|c| c as u8);
    let steps = string(" step").followed_by(token('s').optional());

    let swap_pos = chain((string("swap position "), position, string(" with position "), position))
        .map(|(_, x, _, y)| Operation::SwapPositions { x, y });
    let swap_letters = chain((string("swap letter "), letter, string(" with letter "), letter))
        .map(|(_, x, _, y)| Operation::SwapLetters { x, y });
    let rotate_left = chain((string("rotate left "), parser::usize(), steps))
        .map(|(_, by, _)| Operation::RotateLeft { by });
    let rotate_right = chain((string("rotate right "), parser::usize(), steps))
        .map(|(_, by, _)| Operation::RotateRight { by });
    let move_letter =
        chain((string("move position "), position, string(" to position "), position))
            .map(|(_, from, _, to)| Operation::Move { from, to });
    let reverse = chain((string("reverse positions "), position, string(" through "), position))
        .map(|(_, from, _, through)| Operation::Reverse { from, through });
    let rotate = chain((string("rotate based on position of letter "), letter))
        .map(|(_, based_on)| Operation::Rotate { based_on });

    let operation = swap_pos
        .attempt()
        .or(swap_letters.attempt())
        .or(rotate_left.attempt())
        .or(rotate_right.attempt())
        .or(move_letter.attempt())
        .or(reverse.attempt())
        .or(rotate.attempt());
    operation.collect_sep_by(token('\n')).parse_to_end(&mut input).unwrap()
}

fn part1(operations: &[Operation]) -> [u8; 8] {
    let mut letters = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
    for operation in operations {
        operation.operate(&mut letters);
    }
    letters
}

fn part2(operations: &[Operation]) -> [u8; 8] {
    let mut letters = [b'f', b'b', b'g', b'd', b'c', b'e', b'a', b'h'];
    for operation in operations.iter().rev() {
        operation.reverse().operate(&mut letters);
    }
    letters
}

pub fn solve(input: &str) -> ([u8; 8], [u8; 8]) {
    let operations = parse(input);
    (part1(&operations), part2(&operations))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 21).await?;
    assert_eq!(solve(&input), (*b"ghfacdbe", *b"fhgcdaeb"));
    Ok(())
}
