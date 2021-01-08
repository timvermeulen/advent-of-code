use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Value(i32);

#[derive(Debug, Copy, Clone)]
pub struct Register(usize);

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Value(Value),
    Register(Register),
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Cpy { from: Source, to: Source },
    Inc(Register),
    Dec(Register),
    Jnz { test: Source, by: Source },
    Tgl(Register),
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Cpy { from, to } => Self::Jnz { test: from, by: to },
            Self::Inc(r) => Self::Dec(r),
            Self::Dec(r) => Self::Inc(r),
            Self::Jnz { test, by } => Self::Cpy { from: test, to: by },
            Self::Tgl(r) => Self::Inc(r),
        };
    }
}

pub fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Instruction>> {
    let value = parser::i32().map(|n| Value(n));
    let register = parser::satisfy(|c: char| c.is_ascii()).and_then(|c| {
        let b = c as u8;
        if (b'a'..=b'd').contains(&b) {
            Some(Register((b - b'a') as usize))
        } else {
            None
        }
    });
    let source = choice((
        value.map(|v| Source::Value(v)),
        register.map(|r| Source::Register(r)),
    ));

    let cpy = chain((string("cpy "), source, token(' '), source))
        .map(|(_, from, _, to)| Instruction::Cpy { from, to });
    let inc = chain((string("inc "), register)).map(|(_, r)| Instruction::Inc(r));
    let dec = chain((string("dec "), register)).map(|(_, r)| Instruction::Dec(r));
    let jnz = chain((string("jnz "), source, token(' '), source))
        .map(|(_, test, _, by)| Instruction::Jnz { test, by });
    let tgl = chain((string("tgl "), register)).map(|(_, r)| Instruction::Tgl(r));

    let instruction = choice((cpy, inc, dec, jnz, tgl));
    instruction.collect_sep_by(token('\n'))
}

#[derive(Debug)]
pub struct Regs(pub [i32; 4]);

impl Regs {
    pub fn read(&self, source: Source) -> i32 {
        match source {
            Source::Value(v) => v.0,
            Source::Register(r) => self.0[r.0],
        }
    }

    pub fn write(&mut self, register: Register) -> &mut i32 {
        &mut self.0[register.0]
    }
}

#[derive(Debug)]
pub struct VM {
    pub instructions: Vec<Instruction>,
    pub ip: usize,
    pub regs: Regs,
}

impl VM {
    pub fn run(&mut self) {
        while self.ip < self.instructions.len() {
            let instruction = self.instructions[self.ip];

            match instruction {
                Instruction::Cpy { from, to } => {
                    let value = self.regs.read(from);
                    match to {
                        Source::Value(_) => { /* skip */ }
                        Source::Register(r) => *self.regs.write(r) = value,
                    }
                }
                Instruction::Inc(r) => *self.regs.write(r) += 1,
                Instruction::Dec(r) => *self.regs.write(r) -= 1,
                Instruction::Jnz { test, by } => {
                    if self.regs.read(test) > 0 {
                        let new = self.ip as i32 + self.regs.read(by);
                        if new < 0 {
                            break;
                        } else {
                            self.ip = new as usize;
                            continue;
                        }
                    }
                }
                Instruction::Tgl(r) => {
                    let at = self.ip as i32 + self.regs.read(Source::Register(r));
                    if at >= 0 && (at as usize) < self.instructions.len() {
                        self.instructions[at as usize].toggle();
                    }
                }
            }

            self.ip += 1;
        }
    }
}
