pub struct State<I, O> {
    pub data: Vec<i32>,
    pub pc: i32,
    input: I,
    output: O,
}

impl<I, O> State<I, O>
where
    I: FnMut() -> i32,
    O: FnMut(i32),
{
    pub fn new(data: Vec<i32>, input: I, output: O) -> Self {
        Self { data, pc: 0, input, output }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.data[self.pc as usize];
            self.pc += 1;
            let opcode = Opcode::new(instruction % 100);
            let parameter_modes = ParameterModes { n: instruction / 100 };
            let mut handle = Handle { state: self, parameter_modes };
            match opcode.operate(&mut handle) {
                Action::Continue => {}
                Action::Output(x) => (self.output)(x),
                Action::Halt => return,
            }
        }
    }

    fn read(&mut self, mode: ParameterMode) -> i32 {
        let value = self.data[self.pc as usize];
        self.pc += 1;
        match mode {
            ParameterMode::Position => self.data[value as usize],
            ParameterMode::Immediate => value,
        }
    }

    fn write(&mut self, value: i32) {
        let address = self.read(ParameterMode::Immediate);
        self.data[address as usize] = value;
    }

    fn write_input(&mut self) {
        let input = (self.input)();
        self.write(input)
    }

    fn jump_to(&mut self, address: i32) {
        self.pc = address;
    }
}

struct Handle<'a, I, O> {
    state: &'a mut State<I, O>,
    parameter_modes: ParameterModes,
}

impl<I, O> Handle<'_, I, O>
where
    I: FnMut() -> i32,
    O: FnMut(i32),
{
    fn read(&mut self) -> i32 {
        self.state.read(self.parameter_modes.next())
    }

    fn write(&mut self, value: i32) {
        self.state.write(value)
    }

    fn write_input(&mut self) {
        self.state.write_input()
    }

    fn jump_to(&mut self, address: i32) {
        self.state.jump_to(address)
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

    fn operate<I, O>(self, handle: &mut Handle<'_, I, O>) -> Action
    where
        I: FnMut() -> i32,
        O: FnMut(i32),
    {
        match self {
            Self::Add => {
                let sum = handle.read() + handle.read();
                handle.write(sum);
            }
            Self::Multiply => {
                let product = handle.read() * handle.read();
                handle.write(product);
            }
            Self::Input => handle.write_input(),
            Self::Output => return Action::Output(handle.read()),
            Self::JumpTrue => {
                let condition = handle.read() != 0;
                let address = handle.read();
                if condition {
                    handle.jump_to(address);
                }
            }
            Self::JumpFalse => {
                let condition = handle.read() == 0;
                let address = handle.read();
                if condition {
                    handle.jump_to(address);
                }
            }
            Self::LessThan => {
                let first = handle.read();
                let second = handle.read();
                handle.write(if first < second { 1 } else { 0 });
            }
            Self::EqualTo => {
                let first = handle.read();
                let second = handle.read();
                handle.write(if first == second { 1 } else { 0 });
            }
            Self::Halt => return Action::Halt,
        }
        Action::Continue
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

struct ParameterModes {
    n: i32,
}

impl ParameterModes {
    fn next(&mut self) -> ParameterMode {
        let mode = match self.n % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            k => panic!("invalid parameter mode: {}", k),
        };
        self.n /= 10;
        mode
    }
}

enum Action {
    Continue,
    Output(i32),
    Halt,
}
