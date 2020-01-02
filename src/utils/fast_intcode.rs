use std::{hint::unreachable_unchecked, intrinsics, iter};

#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<i64>,
    pub pc: usize,
    pub base: i64,
    pub state: State,
}

pub struct Iter<'a, I>
where
    I: Iterator<Item = i64>,
{
    inputs: I,
    comp: &'a mut Computer,
}

impl<I> Iterator for Iter<'_, I>
where
    I: Iterator<Item = i64>,
{
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.comp.step_with_iter(&mut self.inputs).output()
    }
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self { memory, pc: 0, base: 0, state: State::Idle }
    }

    pub fn run(&mut self) -> Iter<'_, iter::Empty<i64>> {
        self.run_with_iter(iter::empty())
    }

    pub fn run_with(&mut self, input: i64) -> Iter<'_, iter::Once<i64>> {
        self.run_with_iter(iter::once(input))
    }

    pub fn run_with_iter<I>(&mut self, inputs: I) -> Iter<'_, I::IntoIter>
    where
        I: IntoIterator<Item = i64>,
    {
        Iter { inputs: inputs.into_iter(), comp: self }
    }

    pub fn step(&mut self) -> Interrupt {
        debug_assert!(
            !matches!(self.state, State::WaitingForInput { .. }),
            "cant step, waiting for input"
        );
        loop {
            match self.single_instruction() {
                InstructionState::Idle => {}
                InstructionState::Halt => {
                    self.state = State::Halted;
                    return Interrupt::Halt;
                }
                InstructionState::NeedsInput { address } => {
                    self.state = State::WaitingForInput { address };
                    return Interrupt::WaitingForInput;
                }
                InstructionState::Output(i64) => return Interrupt::Output(i64),
            }
        }
    }

    pub fn step_with(&mut self, input: i64) -> Interrupt {
        self.step_with_iter(Some(input))
    }

    pub fn single_instruction(&mut self) -> InstructionState {
        debug_assert!(self.pc < self.memory.len());
        let instruction = *unsafe { self.memory.get_unchecked(self.pc) };

        macro_rules! read_imm {
            ($i:expr) => {
                *unsafe { self.memory.get_unchecked(self.pc + $i as usize) }
            };
        }

        macro_rules! read_pos {
            ($i: expr) => {{
                let address = read_imm!($i);
                self.memory.get(address as usize).copied().unwrap_or(0)
            }};
        }

        macro_rules! read_rel {
            ($i: expr) => {{
                let address = self.base + read_imm!($i);
                self.memory.get(address as usize).copied().unwrap_or(0)
            }};
        }

        macro_rules! write_pos {
            ($val: expr, $i: expr) => {{
                let address = read_imm!($i) as usize;
                if intrinsics::unlikely(address >= self.memory.len()) {
                    self.extend(address);
                }
                *unsafe { self.memory.get_unchecked_mut(address) } = $val;
            }};
        }

        macro_rules! write_rel {
            ($val: expr, $i: expr) => {{
                let address = (self.base + read_imm!($i)) as usize;
                if intrinsics::unlikely(address >= self.memory.len()) {
                    self.extend(address);
                }
                *unsafe { self.memory.get_unchecked_mut(address) } = $val;
            }};
        }

        match instruction {
            000_01 => {
                let a = read_pos!(1);
                let b = read_pos!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            001_01 => {
                let a = read_imm!(1);
                let b = read_pos!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            002_01 => {
                let a = read_rel!(1);
                let b = read_pos!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            010_01 => {
                let a = read_pos!(1);
                let b = read_imm!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            011_01 => {
                let a = read_imm!(1);
                let b = read_imm!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            012_01 => {
                let a = read_rel!(1);
                let b = read_imm!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            020_01 => {
                let a = read_pos!(1);
                let b = read_rel!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            021_01 => {
                let a = read_imm!(1);
                let b = read_rel!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            022_01 => {
                let a = read_rel!(1);
                let b = read_rel!(2);
                write_pos!(a + b, 3);
                self.pc += 4;
            }
            200_01 => {
                let a = read_pos!(1);
                let b = read_pos!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            201_01 => {
                let a = read_imm!(1);
                let b = read_pos!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            211_01 => {
                let a = read_imm!(1);
                let b = read_imm!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            210_01 => {
                let a = read_pos!(1);
                let b = read_imm!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            212_01 => {
                let a = read_rel!(1);
                let b = read_imm!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            220_01 => {
                let a = read_pos!(1);
                let b = read_rel!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            221_01 => {
                let a = read_imm!(1);
                let b = read_rel!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            222_01 => {
                let a = read_rel!(1);
                let b = read_rel!(2);
                write_rel!(a + b, 3);
                self.pc += 4;
            }
            000_02 => {
                let a = read_pos!(1);
                let b = read_pos!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            001_02 => {
                let a = read_imm!(1);
                let b = read_pos!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            010_02 => {
                let a = read_pos!(1);
                let b = read_imm!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            011_02 => {
                let a = read_imm!(1);
                let b = read_imm!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            012_02 => {
                let a = read_rel!(1);
                let b = read_imm!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            021_02 => {
                let a = read_imm!(1);
                let b = read_rel!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            022_02 => {
                let a = read_rel!(1);
                let b = read_rel!(2);
                write_pos!(a * b, 3);
                self.pc += 4;
            }
            200_02 => {
                let a = read_pos!(1);
                let b = read_pos!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            201_02 => {
                let a = read_imm!(1);
                let b = read_pos!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            210_02 => {
                let a = read_pos!(1);
                let b = read_imm!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            211_02 => {
                let a = read_imm!(1);
                let b = read_imm!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            212_02 => {
                let a = read_rel!(1);
                let b = read_imm!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            220_02 => {
                let a = read_pos!(1);
                let b = read_rel!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            221_02 => {
                let a = read_imm!(1);
                let b = read_rel!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            222_02 => {
                let a = read_rel!(1);
                let b = read_rel!(2);
                write_rel!(a * b, 3);
                self.pc += 4;
            }
            0_03 => {
                let address = read_imm!(1) as usize;
                self.pc += 2;
                return InstructionState::NeedsInput { address };
            }
            2_03 => {
                let address = (self.base + read_imm!(1)) as usize;
                self.pc += 2;
                return InstructionState::NeedsInput { address };
            }
            0_04 => {
                let output = read_pos!(1);
                self.pc += 2;
                return InstructionState::Output(output);
            }
            1_04 => {
                let output = read_imm!(1);
                self.pc += 2;
                return InstructionState::Output(output);
            }
            2_04 => {
                let output = read_rel!(1);
                self.pc += 2;
                return InstructionState::Output(output);
            }
            000_05 => {
                let x = read_pos!(1);
                let address = read_pos!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            001_05 => {
                let x = read_imm!(1);
                let address = read_pos!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            010_05 => {
                let x = read_pos!(1);
                let address = read_imm!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            011_05 => {
                let x = read_imm!(1);
                let address = read_imm!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            012_05 => {
                let x = read_rel!(1);
                let address = read_imm!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            021_05 => {
                let x = read_imm!(1);
                let address = read_rel!(2);
                self.pc = if x != 0 { address as usize } else { self.pc + 3 };
            }
            000_06 => {
                let x = read_pos!(1);
                let address = read_pos!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            001_06 => {
                let x = read_imm!(1);
                let address = read_pos!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            010_06 => {
                let x = read_pos!(1);
                let address = read_imm!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            011_06 => {
                let x = read_imm!(1);
                let address = read_imm!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            012_06 => {
                let x = read_rel!(1);
                let address = read_imm!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            021_06 => {
                let x = read_imm!(1);
                let address = read_rel!(2);
                self.pc = if x == 0 { address as usize } else { self.pc + 3 };
            }
            000_07 => {
                let x = read_pos!(1);
                let y = read_pos!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            001_07 => {
                let x = read_imm!(1);
                let y = read_pos!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            010_07 => {
                let x = read_pos!(1);
                let y = read_imm!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            011_07 => {
                let x = read_imm!(1);
                let y = read_imm!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            012_07 => {
                let x = read_rel!(1);
                let y = read_imm!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            021_07 => {
                let x = read_imm!(1);
                let y = read_rel!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            022_07 => {
                let x = read_rel!(1);
                let y = read_rel!(2);
                write_pos!((x < y) as i64, 3);
                self.pc += 4;
            }
            201_07 => {
                let x = read_imm!(1);
                let y = read_pos!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            202_07 => {
                let x = read_rel!(1);
                let y = read_pos!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            210_07 => {
                let x = read_pos!(1);
                let y = read_imm!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            211_07 => {
                let x = read_imm!(1);
                let y = read_imm!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            212_07 => {
                let x = read_rel!(1);
                let y = read_imm!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            220_07 => {
                let x = read_pos!(1);
                let y = read_rel!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            221_07 => {
                let x = read_imm!(1);
                let y = read_rel!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            222_07 => {
                let x = read_rel!(1);
                let y = read_rel!(2);
                write_rel!((x < y) as i64, 3);
                self.pc += 4;
            }
            000_08 => {
                let x = read_pos!(1);
                let y = read_pos!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            001_08 => {
                let x = read_imm!(1);
                let y = read_pos!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            002_08 => {
                let x = read_rel!(1);
                let y = read_pos!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            010_08 => {
                let x = read_pos!(1);
                let y = read_imm!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            011_08 => {
                let x = read_imm!(1);
                let y = read_imm!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            012_08 => {
                let x = read_rel!(1);
                let y = read_imm!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            021_08 => {
                let x = read_imm!(1);
                let y = read_rel!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            022_08 => {
                let x = read_rel!(1);
                let y = read_rel!(2);
                write_pos!((x == y) as i64, 3);
                self.pc += 4;
            }
            200_08 => {
                let x = read_pos!(1);
                let y = read_pos!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            202_08 => {
                let x = read_rel!(1);
                let y = read_pos!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            210_08 => {
                let x = read_pos!(1);
                let y = read_imm!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            211_08 => {
                let x = read_imm!(1);
                let y = read_imm!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            212_08 => {
                let x = read_rel!(1);
                let y = read_imm!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            222_08 => {
                let x = read_rel!(1);
                let y = read_rel!(2);
                write_rel!((x == y) as i64, 3);
                self.pc += 4;
            }
            0_09 => {
                let offset = read_pos!(1);
                self.base += offset;
                self.pc += 2;
            }
            1_09 => {
                let offset = read_imm!(1);
                self.base += offset;
                self.pc += 2;
            }
            2_09 => {
                let offset = read_rel!(1);
                self.base += offset;
                self.pc += 2;
            }
            99 => {
                self.state = State::Halted;
                self.pc += 1;
                return InstructionState::Halt;
            }
            _ => {
                debug_assert!(false, "invalid instruction {}", instruction);
                unsafe { unreachable_unchecked() }
            }
        }
        InstructionState::Idle
    }

    pub fn write_input(&mut self, input: i64) {
        debug_assert!(matches!(self.state, State::WaitingForInput { .. }), "not waiting for input");
        let address = match self.state {
            State::WaitingForInput { address } => address,
            _ => unsafe { unreachable_unchecked() },
        };
        self.state = State::Idle;
        self.write(input, address);
    }

    pub fn step_with_iter(&mut self, input: impl IntoIterator<Item = i64>) -> Interrupt {
        let mut inputs = input.into_iter();

        match self.state {
            State::Halted => {
                self.state = State::Idle;
            }
            State::WaitingForInput { address } => {
                let input = match inputs.next() {
                    None => return Interrupt::WaitingForInput,
                    Some(input) => input,
                };
                self.state = State::Idle;
                self.write(input, address);
            }
            State::Idle => {}
        }
        loop {
            match self.single_instruction() {
                InstructionState::Idle => {}
                InstructionState::Halt => return Interrupt::Halt,
                InstructionState::NeedsInput { address } => match inputs.next() {
                    Some(input) => {
                        self.write(input, address);
                    }
                    None => {
                        self.state = State::WaitingForInput { address };
                        return Interrupt::WaitingForInput;
                    }
                },
                InstructionState::Output(output) => return Interrupt::Output(output),
            }
        }
    }

    #[inline(always)]
    pub fn write(&mut self, value: i64, address: usize) {
        if intrinsics::unlikely(address >= self.memory.len()) {
            self.extend(address);
        }

        debug_assert!(address < self.memory.len());
        *unsafe { self.memory.get_unchecked_mut(address) } = value;
    }

    #[inline(never)]
    fn extend(&mut self, address: usize) {
        self.memory.extend(iter::repeat(0).take(address - self.memory.len() + 1));
    }
}

pub enum InstructionState {
    Idle,
    Halt,
    NeedsInput { address: usize },
    Output(i64),
}

#[derive(Debug)]
pub enum Interrupt {
    Halt,
    WaitingForInput,
    Output(i64),
}

impl Interrupt {
    pub fn output(self) -> Option<i64> {
        match self {
            Self::Halt | Self::WaitingForInput => None,
            Self::Output(output) => Some(output),
        }
    }

    pub fn unwrap(self) -> i64 {
        debug_assert!(!matches!(self, Self::Halt), "no output, VM is halted");
        debug_assert!(!matches!(self, Self::WaitingForInput), "no output, VM is waiting for input");
        match self {
            Self::Output(output) => output,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn is_halt(self) -> bool {
        match self {
            Self::Halt => true,
            Self::WaitingForInput | Self::Output(_) => false,
        }
    }

    pub fn needs_input(self) -> bool {
        match self {
            Self::Halt | Self::Output(_) => false,
            Self::WaitingForInput => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum State {
    Idle,
    Halted,
    WaitingForInput { address: usize },
}

#[derive(Copy, Clone)]
pub enum Mode {
    Pos,
    Imm,
    Rel,
}

#[derive(Copy, Clone)]
pub enum WriteMode {
    Pos,
    Rel,
}
