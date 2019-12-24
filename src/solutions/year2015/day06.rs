use super::*;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Instruction>> {
    let coordinates =
        chain((parser::u32(), token(','), parser::u32())).map(|(a, _, b)| (a as usize, b as usize));

    let action = choice((
        string("turn on").attempt().map(|_| Action::TurnOn),
        string("turn off").attempt().map(|_| Action::TurnOff),
        string("toggle").attempt().map(|_| Action::Toggle),
    ));

    let instruction = chain((action, token(' '), coordinates, string(" through "), coordinates))
        .map(|(action, _, start, _, end)| Instruction { action, start, end });

    instruction.collect_sep_by(token('\n'))
}

const SIZE: usize = 1000;

fn part1(instructions: &[Instruction]) -> usize {
    // TODO: dedicated grid type
    let mut grid = [[false; SIZE]; SIZE];

    for instruction in instructions {
        for col in grid.iter_mut().take(instruction.end.0 + 1).skip(instruction.start.0) {
            for light in col.iter_mut().take(instruction.end.1 + 1).skip(instruction.start.1) {
                match instruction.action {
                    Action::TurnOn => *light = true,
                    Action::TurnOff => *light = false,
                    Action::Toggle => *light = !*light,
                }
            }
        }
    }

    grid.iter().flat_map(|x| x.iter()).copied().filter(|&l| l).count()
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut grid = box [[0_usize; SIZE]; SIZE];

    for instruction in instructions {
        for col in grid.iter_mut().take(instruction.end.0 + 1).skip(instruction.start.0) {
            for light in col.iter_mut().take(instruction.end.1 + 1).skip(instruction.start.1) {
                match instruction.action {
                    Action::TurnOn => *light += 1,
                    Action::TurnOff => {
                        if *light != 0 {
                            *light -= 1
                        }
                    }
                    Action::Toggle => *light += 2,
                }
            }
        }
    }

    grid.iter().flat_map(|x| x.iter()).copied().sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let instructions = parser().parse_to_end(&input).unwrap();
    (part1(&instructions), part2(&instructions))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 6).await?;
    let instructions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&instructions), 569_999);
    assert_eq!(part2(&instructions), 17_836_115);
    Ok(())
}
