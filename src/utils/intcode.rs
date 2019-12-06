pub struct Computer {
    pub memory: Vec<i32>,
    pub pc: i32,
}

impl Computer {
    pub fn new(memory: Vec<i32>) -> Self {
        Self { memory, pc: 0 }
    }

    pub fn run(&mut self) -> State {
        loop {
            let instruction = self.memory[self.pc as usize];
            self.pc += 1;
            let opcode = Opcode::new(instruction % 100);
            let modes = ParamModes { n: instruction / 100 };
            let mut handle = Params { comp: self, modes };
            if let Some(state) = opcode.operate(&mut handle) {
                match state {
                    State::WaitingForInput => return State::WaitingForInput,
                    State::Output(x) => return State::Output(x),
                    State::Halt => return State::Halt,
                }
            }
        }
    }

    pub fn run_with_input(&mut self, input: Option<i32>) -> State {
        if let Some(input) = input {
            self.write(input);
        }
        self.run()
    }

    pub fn read(&mut self, mode: ParamMode) -> i32 {
        let value = self.memory[self.pc as usize];
        self.pc += 1;
        match mode {
            ParamMode::Position => self.memory[value as usize],
            ParamMode::Immediate => value,
        }
    }

    pub fn write(&mut self, value: i32) {
        let address = self.read(ParamMode::Immediate);
        self.memory[address as usize] = value;
    }

    pub fn jump_to(&mut self, address: i32) {
        self.pc = address;
    }
}

struct Params<'a> {
    comp: &'a mut Computer,
    modes: ParamModes,
}

impl Params<'_> {
    fn read(&mut self) -> i32 {
        self.comp.read(self.modes.next())
    }

    fn write(&mut self, value: i32) {
        self.comp.write(value);
    }

    fn jump_to(&mut self, address: i32) {
        self.comp.jump_to(address);
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
}

impl Opcode {
    fn new(opcode: i32) -> Self {
        match opcode {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpTrue,
            6 => Self::JumpFalse,
            7 => Self::LessThan,
            8 => Self::EqualTo,
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
            Self::Input => return Some(State::WaitingForInput),
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
            Self::Halt => return Some(State::Halt),
        }
        None
    }
}

pub enum State {
    Halt,
    WaitingForInput,
    Output(i32),
}

pub enum ParamMode {
    Position,
    Immediate,
}

struct ParamModes {
    n: i32,
}

impl ParamModes {
    fn next(&mut self) -> ParamMode {
        let mode = match self.n % 10 {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            k => panic!("invalid parameter mode: {}", k),
        };
        self.n /= 10;
        mode
    }
}
