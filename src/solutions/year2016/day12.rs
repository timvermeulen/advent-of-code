use super::*;
use assembunny::*;

pub fn solve(input: &str) -> (i32, i32) {
    let mut vm = VM {
        instructions: parser().parse_to_end(input).unwrap(),
        ip: 0,
        regs: Regs([0; 4]),
    };

    vm.run();
    let part1 = vm.regs.0[0];

    vm.ip = 0;
    vm.regs = Regs([0, 0, 1, 0]);
    vm.run();
    let part2 = vm.regs.0[0];

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 12).await?;
    assert_eq!(solve(&input), (317_993, 9_227_647));
    Ok(())
}
