use super::*;

fn parse(input: &str) -> Vec<usize> {
    parser::usize().collect_sep_by(comma()).parse_to_end(&input).unwrap()
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn operate(self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

enum Opcode {
    Operation(Operation),
    End,
}

impl Opcode {
    fn new(n: usize) -> Self {
        match n {
            1 => Self::Operation(Operation::Add),
            2 => Self::Operation(Operation::Multiply),
            99 => Self::End,
            _ => unreachable!(),
        }
    }
}

fn run(mut nums: Vec<usize>, noun: usize, verb: usize) -> usize {
    nums[1] = noun;
    nums[2] = verb;
    let mut address = 0;

    for address in (0..).step_by(4) {
        match Opcode::new(nums[address]) {
            Opcode::Operation(op) => {
                if let [source1, source2, dest] = nums[address + 1..address + 4] {
                    nums[dest] = op.operate(nums[source1], nums[source2]);
                } else {
                    unreachable!()
                }
            }
            Opcode::End => return nums[0],
        }
    }
    unreachable!()
}

fn part1(nums: &[usize]) -> usize {
    run(nums.to_owned(), 12, 2)
}

fn part2(nums: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(nums.to_owned(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 2).await?;
    let nums = parse(&input);
    assert_eq!(part1(&nums), 3_409_710);
    assert_eq!(part2(&nums), 7912);
    Ok(())
}
