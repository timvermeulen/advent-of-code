use parser::prelude::*;
use std::iter;

pub mod prelude {
    pub use super::{Computer, Interrupt};
}

pub fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i64>> {
    parser::i64().collect_sep_by(token(','))
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Computer {
    pub memory: Vec<i64>,
    pub pc: i64,
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

    pub fn needs_input(&self) -> bool {
        match self.state {
            State::Idle | State::Halted => false,
            State::WaitingForInput(_) => true,
        }
    }

    pub fn is_halted(&self) -> bool {
        match self.state {
            State::Idle | State::WaitingForInput(_) => false,
            State::Halted => true,
        }
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
        self.step_with_iter(None)
    }

    pub fn step_with(&mut self, input: i64) -> Interrupt {
        self.step_with_iter(Some(input))
    }

    pub fn step_with_iter(&mut self, input: impl IntoIterator<Item = i64>) -> Interrupt {
        let mut inputs = input.into_iter();

        loop {
            match self.state {
                State::Halted => {
                    println!("warning: the program has been halted before");
                    self.state = State::Idle;
                }
                State::WaitingForInput(mode) => {
                    let input = match inputs.next() {
                        None => return Interrupt::WaitingForInput,
                        Some(input) => input,
                    };
                    self.state = State::Idle;
                    self.write(input, mode);
                }
                State::Idle => {}
            }

            let instruction = self.memory[self.pc as usize];
            self.pc += 1;
            let opcode = Opcode::new(instruction % 100);
            let mut params = Params { comp: self, modes: instruction / 100 };

            match opcode {
                Opcode::Add => {
                    let sum = params.read() + params.read();
                    params.write(sum);
                }
                Opcode::Multiply => {
                    let product = params.read() * params.read();
                    params.write(product);
                }
                Opcode::Input => self.state = State::WaitingForInput(params.next_mode()),
                Opcode::Output => return Interrupt::Output(params.read()),
                Opcode::JumpTrue => {
                    let condition = params.read() != 0;
                    let address = params.read();
                    if condition {
                        self.pc = address;
                    }
                }
                Opcode::JumpFalse => {
                    let condition = params.read() == 0;
                    let address = params.read();
                    if condition {
                        self.pc = address;
                    }
                }
                Opcode::LessThan => {
                    let first = params.read();
                    let second = params.read();
                    params.write((first < second) as i64);
                }
                Opcode::EqualTo => {
                    let first = params.read();
                    let second = params.read();
                    params.write((first == second) as i64);
                }
                Opcode::Adjust => {
                    let offset = params.read();
                    self.base += offset;
                }
                Opcode::Halt => {
                    self.state = State::Halted;
                    return Interrupt::Halt;
                }
            }
        }
    }

    pub fn read(&mut self, mode: ParamMode) -> i64 {
        let value = self.memory[self.pc as usize];
        self.pc += 1;

        let address = match mode {
            ParamMode::Position => value,
            ParamMode::Immediate => return value,
            ParamMode::Relative => self.base + value,
        } as usize;

        self.memory.get(address).copied().unwrap_or(0)
    }

    pub fn write(&mut self, value: i64, mode: ParamMode) {
        let address = self.read(ParamMode::Immediate);

        let address = match mode {
            ParamMode::Position => address,
            ParamMode::Immediate => panic!("writing cannot happen in immediate mode"),
            ParamMode::Relative => address + self.base,
        } as usize;

        if address as usize >= self.memory.len() {
            self.memory.extend(std::iter::repeat(0).take(address as usize - self.memory.len() + 1));
        }

        self.memory[address as usize] = value;
    }
}

struct Params<'a> {
    comp: &'a mut Computer,
    modes: i64,
}

impl Params<'_> {
    fn read(&mut self) -> i64 {
        let mode = self.next_mode();
        self.comp.read(mode)
    }

    fn write(&mut self, value: i64) {
        let mode = self.next_mode();
        self.comp.write(value, mode);
    }

    fn next_mode(&mut self) -> ParamMode {
        let mode = ParamMode::new(self.modes % 10);
        self.modes /= 10;
        mode
    }
}

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    EqualTo,
    Halt,
    Adjust,
}

impl Opcode {
    fn new(opcode: i64) -> Self {
        match opcode {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpTrue,
            6 => Self::JumpFalse,
            7 => Self::LessThan,
            8 => Self::EqualTo,
            9 => Self::Adjust,
            99 => Self::Halt,
            _ => panic!("invalid opcode: {}", opcode),
        }
    }
}

#[derive(Debug, PartialEq)]
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
        self.output().unwrap()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Idle,
    Halted,
    WaitingForInput(ParamMode),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn new(n: i64) -> Self {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            k => panic!("invalid parameter mode: {}", k),
        }
    }
}
