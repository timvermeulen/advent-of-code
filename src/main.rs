#![allow(unused)]
#![feature(
    test,
    array_value_iter,
    try_blocks,
    box_syntax,
    core_intrinsics,
    partition_point
)]

mod input;
mod solutions;
mod utils;

use solutions::*;

// #[async_std::main]
// async fn main() -> Result<(), InputError> {
// let input = get_input(2019, 7).await?;
// dbg!(year2019_optimized::day07::solve(&input));
// Ok(())
// }

fn main() {
    // Opcode::Add(m0, m1, m2) => {
    //     let a = self.read(m0, 1);
    //     let b = self.read(m1, 2);
    //     let address = self.read(Mode::Imm, 3);
    //     let address = match m2 {
    //         WriteMode::Pos => address,
    //         WriteMode::Rel => address + self.base,
    //     } as usize;
    //     self.write(a + b, address);
    //     self.pc += 4;
    //     InstructionState::Idle
    // }

    for &instruction in &[
        000_01, 001_01, 002_01, 010_01, 011_01, 012_01, 020_01, 021_01, 022_01, 200_01, 201_01,
        211_01, 210_01, 212_01, 220_01, 221_01, 222_01,
    ] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let a = read_{}!(1);", mode());
        println!("let b = read_{}!(2);", mode());

        println!("write_{}!(a + b, 3);", mode());
        println!("self.pc += 4;");
        println!("InstructionState::Idle");

        println!("}}");
    }

    for &instruction in &[
        000_02, 001_02, 010_02, 011_02, 012_02, 021_02, 022_02, 200_02, 201_02, 210_02, 211_02,
        212_02, 220_02, 221_02, 222_02,
    ] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let a = read_{}!(1);", mode());
        println!("let b = read_{}!(2);", mode());

        println!("write_{}!(a * b, 3);", mode());
        println!("self.pc += 4;");
        println!("InstructionState::Idle");

        println!("}}");
    }

    for &instruction in &[00_05, 01_05, 10_05, 11_05, 12_05, 21_05] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let x = read_{}!(1);", mode());
        println!("let address = read_{}!(2);", mode());
        println!("self.pc = if x != 0 {{ address as usize }} else {{ self.pc + 3 }};");
        println!("InstructionState::Idle");

        println!("}}");
    }

    for &instruction in &[00_06, 01_06, 10_06, 11_06, 12_06, 21_06] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let x = read_{}!(1);", mode());
        println!("let address = read_{}!(2);", mode());
        println!("self.pc = if x == 0 {{ address as usize }} else {{ self.pc + 3 }};");
        println!("InstructionState::Idle");

        println!("}}");
    }

    for &instruction in &[
        000_07, 001_07, 010_07, 011_07, 012_07, 021_07, 022_07, 201_07, 202_07, 210_07, 211_07,
        212_07, 220_07, 221_07, 222_07,
    ] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let x = read_{}!(1);", mode());
        println!("let y = read_{}!(2);", mode());
        println!("write_{}!((x < y) as i64, 3);", mode());
        println!("self.pc += 4;");
        println!("InstructionState::Idle");

        println!("}}");
    }

    for &instruction in &[
        000_08, 001_08, 002_08, 010_08, 011_08, 012_08, 021_08, 022_08, 200_08, 202_08, 210_08,
        211_08, 212_08, 222_08,
    ] {
        let opcode = instruction % 100;
        let mut params = instruction / 100;
        println!("{:03}_{:02} => {{", params, opcode);

        let mut param = || {
            let p = params % 10;
            params /= 10;
            p
        };
        let mut mode = || match param() {
            0 => "pos",
            1 => "imm",
            2 => "rel",
            _ => unreachable!(),
        };

        println!("let x = read_{}!(1);", mode());
        println!("let y = read_{}!(2);", mode());
        println!("write_{}!((x == y) as i64, 3);", mode());
        println!("self.pc += 4;");
        println!("InstructionState::Idle");

        println!("}}");
    }
}
