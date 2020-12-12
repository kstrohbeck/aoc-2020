use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map_res, value},
    IResult,
};

pub fn star_1(data: String) {
    let instructions = parse(&data);
    let mut has_visited = vec![false; instructions.len()];
    let mut program = ProgramState::new(instructions);

    loop {
        if has_visited[program.pc] {
            break;
        }
        has_visited[program.pc] = true;
        match program.step() {
            StepResult::Continue => {}
            StepResult::Terminate => println!("Terminated."),
            StepResult::InstructionOutOfRange => {
                println!("Instruction out of range at pc {}", program.pc)
            }
        }
    }

    println!("{}", program.acc);
}

pub fn star_2(data: String) {
    let instructions = parse(&data);
    let instructions_len = instructions.len();
    let mut program = ProgramState::new(instructions);

    'a: for i in 0..instructions_len {
        program.reset();

        let instruction = program.instructions[i];
        program.patch = match instruction.opcode {
            Opcode::Nop => Some((
                i,
                Instruction {
                    opcode: Opcode::Jmp,
                    argument: instruction.argument,
                },
            )),
            Opcode::Jmp => Some((
                i,
                Instruction {
                    opcode: Opcode::Nop,
                    argument: instruction.argument,
                },
            )),
            Opcode::Acc => continue,
        };

        let mut has_visited = vec![false; instructions_len];

        loop {
            if let Some(hv) = has_visited.get_mut(program.pc) {
                if *hv {
                    continue 'a;
                }
                *hv = true;
            }

            match program.step() {
                StepResult::Continue => {}
                StepResult::Terminate => break 'a,
                StepResult::InstructionOutOfRange => continue 'a,
            }
        }
    }

    println!("acc: {}", program.acc);
    println!("patch: {:?}", program.patch);
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| instruction(s).unwrap().1)
        .collect()
}

#[derive(Debug, Clone)]
struct ProgramState {
    pc: usize,
    acc: i32,
    instructions: Vec<Instruction>,
    patch: Option<(usize, Instruction)>,
}

impl ProgramState {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            acc: 0,
            instructions,
            patch: None,
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    fn get_instruction(&self) -> Option<Instruction> {
        if let Some((pc, instruction)) = self.patch {
            if self.pc == pc {
                return Some(instruction);
            }
        }

        self.instructions.get(self.pc).copied()
    }

    fn step(&mut self) -> StepResult {
        if self.pc == self.instructions.len() {
            return StepResult::Terminate;
        }

        let instruction = match self.get_instruction() {
            Some(i) => i,
            None => return StepResult::InstructionOutOfRange,
        };

        match instruction.opcode {
            Opcode::Nop => self.pc += 1,
            Opcode::Acc => {
                self.acc += instruction.argument;
                self.pc += 1;
            }
            Opcode::Jmp => {
                let next_pc = self.pc as isize + instruction.argument as isize;
                self.pc = next_pc as usize;
            }
        }

        StepResult::Continue
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepResult {
    Continue,
    Terminate,
    InstructionOutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    argument: i32,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, opcode) = opcode(input)?;
    let (input, _) = space1(input)?;
    let (input, argument) = argument(input)?;
    Ok((input, Instruction { opcode, argument }))
}

fn argument(input: &str) -> IResult<&str, i32> {
    let (input, sign) = alt((value(1, tag("+")), value(-1, tag("-"))))(input)?;
    let (input, amount) = map_res(digit1, |d: &str| d.parse::<u32>())(input)?;
    Ok((input, sign * amount as i32))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}

fn opcode(input: &str) -> IResult<&str, Opcode> {
    alt((
        value(Opcode::Nop, tag("nop")),
        value(Opcode::Acc, tag("acc")),
        value(Opcode::Jmp, tag("jmp")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::{instruction, Instruction, Opcode};

    #[test]
    fn instruction_parses_nop() {
        assert_eq!(
            instruction("nop +0..."),
            Ok((
                "...",
                Instruction {
                    opcode: Opcode::Nop,
                    argument: 0,
                }
            )),
        );
    }

    #[test]
    fn instruction_parses_acc() {
        assert_eq!(
            instruction("acc +1..."),
            Ok((
                "...",
                Instruction {
                    opcode: Opcode::Acc,
                    argument: 1,
                }
            )),
        );
    }

    #[test]
    fn instruction_parses_jmp() {
        assert_eq!(
            instruction("jmp -3..."),
            Ok((
                "...",
                Instruction {
                    opcode: Opcode::Jmp,
                    argument: -3,
                }
            )),
        );
    }
}
