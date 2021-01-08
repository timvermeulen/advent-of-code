use super::*;
use assembunny::*;

pub fn solve(input: &str) {
    let instructions = parser().parse_to_end(input).unwrap();

    let mut vm = VM {
        instructions: instructions.clone(),
        ip: 0,
        regs: Regs([7, 0, 0, 0]),
    };
    vm.run();
    println!("part 1: {}", vm.regs.0[0]);

    let mut vm = VM {
        instructions,
        ip: 0,
        regs: Regs([12, 0, 0, 0]),
    };
    vm.run();
    println!("part 2: {}", vm.regs.0[0]);
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 23).await?;
    // assert_eq!(solve(&input), (317_993, 9_227_647));
    solve(&input);
    Ok(())
}
