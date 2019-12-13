use std::convert::TryFrom;

pub struct Computer {
    pub memory: Vec<i64>,
    pub pc: i64,
    pub base: i64,
    pub input_mode: Option<ParamMode>,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self { memory, pc: 0, base: 0, input_mode: None }
    }

    pub fn run(&mut self) -> State {
        self.run_with(std::iter::empty())
    }

    pub fn run_with(&mut self, input: impl IntoIterator<Item = i64>) -> State {
        let mut inputs = input.into_iter();

        loop {
            if let Some(mode) = self.input_mode {
                let input = match inputs.next() {
                    None => return State::WaitingForInput,
                    Some(input) => input,
                };
                self.input_mode = None;
                self.write(input, mode);
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
                Opcode::Input => self.input_mode = Some(params.next_mode()),
                Opcode::Output => return State::Output(params.read()),
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
                Opcode::Halt => return State::Halt,
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
pub enum State {
    Halt,
    WaitingForInput,
    Output(i64),
}

impl State {
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

#[derive(Debug, Copy, Clone)]
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
