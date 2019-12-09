use std::convert::TryFrom;

pub struct Computer {
    pub memory: Vec<i64>,
    pub pc: i64,
    pub base: i64,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self { memory, pc: 0, base: 0 }
    }

    pub fn run(&mut self) -> State {
        loop {
            let instruction = self.memory[self.pc as usize];
            self.pc += 1;
            let opcode = Opcode::new(instruction % 100);
            let modes = ParamModes { n: instruction / 100 };
            let mut handle = Params { comp: self, modes };
            if let Some(state) = opcode.operate(&mut handle) {
                return state;
            }
        }
    }

    pub fn run_with_input(&mut self, input: Option<i64>) -> State {
        if let Some(input) = input {
            let mode = ParamMode::new(self.memory[self.pc as usize - 1] / 100);
            self.write(input, mode);
        }
        self.run()
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
    modes: ParamModes,
}

impl Params<'_> {
    fn read(&mut self) -> i64 {
        self.comp.read(self.modes.next())
    }

    fn write(&mut self, value: i64) {
        let mode = self.modes.next();
        self.comp.write(value, mode);
    }

    fn jump_to(&mut self, address: i64) {
        self.comp.pc = address;
    }

    fn adjust_by(&mut self, offset: i64) {
        self.comp.base += offset;
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

    fn operate(self, params: &mut Params<'_>) -> Option<State> {
        match self {
            Self::Add => {
                let sum = params.read() + params.read();
                params.write(sum);
            }
            Self::Multiply => {
                let product = params.read() * params.read();
                params.write(product);
            }
            Self::Input => {
                return Some(State::WaitingForInput);
            }
            Self::Output => return Some(State::Output(params.read())),
            Self::JumpTrue => {
                let condition = params.read() != 0;
                let address = params.read();
                if condition {
                    params.jump_to(address);
                }
            }
            Self::JumpFalse => {
                let condition = params.read() == 0;
                let address = params.read();
                if condition {
                    params.jump_to(address);
                }
            }
            Self::LessThan => {
                let first = params.read();
                let second = params.read();
                params.write(if first < second { 1 } else { 0 });
            }
            Self::EqualTo => {
                let first = params.read();
                let second = params.read();
                params.write(if first == second { 1 } else { 0 });
            }
            Self::Adjust => {
                let offset = params.read();
                params.adjust_by(offset);
            }
            Self::Halt => return Some(State::Halt),
        }
        None
    }
}

#[derive(Debug)]
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

struct ParamModes {
    n: i64,
}

impl ParamModes {
    fn next(&mut self) -> ParamMode {
        let mode = ParamMode::new(self.n % 10);
        self.n /= 10;
        mode
    }
}
