use super::challenge::{Challenge, Error as ChallengeErr};
use std::collections::HashSet;
use std::str::FromStr;

enum Command {
    NOP(i32),
    JMP(i32),
    ACC(i32),
}

impl FromStr for Command {
    type Err = std::num::ParseIntError;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let val = &command[4..].parse()?;

        match &command[..3] {
            "jmp" => Ok(Command::JMP(*val)),
            "acc" => Ok(Command::ACC(*val)),
            "nop" => Ok(Command::NOP(*val)),
            code => panic!("Unsupported command opcode: {}", code),
        }
    }
}

impl Command {
    fn try_to_fix(&self) -> Command {
        match self {
            Command::NOP(val) => Command::JMP(*val),
            Command::JMP(val) => Command::NOP(*val),
            Command::ACC(val) => Command::ACC(*val),
        }
    }
}

type Program = Vec<Command>;

struct Registers {
    acc: i32,
    ip: i32,
}

impl Registers {
    fn new() -> Registers {
        Registers { acc: 0, ip: 0 }
    }
}

struct Emulation<'a> {
    program: &'a Program,
    registers: Registers,
}

impl Emulation<'_> {
    fn new(program: &Program) -> Emulation {
        Emulation {
            program,
            registers: Registers::new(),
        }
    }

    fn run_with_loop_detection(self) -> (bool, Registers) {
        let mut visited_ips: HashSet<i32> = HashSet::new();
        let mut last_state = Registers::new();

        for registers in self {
            if visited_ips.contains(&registers.ip) {
                return (true, registers);
            }

            visited_ips.insert(registers.ip);
            last_state = registers
        }

        (false, last_state)
    }
}

impl Iterator for Emulation<'_> {
    type Item = Registers;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ip = self.registers.ip;
        let mut acc = self.registers.acc;
        if ip as usize >= self.program.len() {
            return None;
        }

        let next_command = &self.program[ip as usize];
        match next_command {
            Command::NOP(_) => {
                ip += 1;
            }
            Command::JMP(jump) => {
                ip += jump;
            }
            Command::ACC(diff) => {
                ip += 1;
                acc += diff;
            }
        }

        self.registers.acc = acc;
        self.registers.ip = ip;

        Some(Registers { acc, ip })
    }
}

fn parse_program(program: &str) -> Program {
    program
        .lines()
        .map(|command| command.parse().unwrap())
        .collect()
}

fn find_acc_value_before_entering_loop(program: &Program) -> Option<i32> {
    let (has_loop, registers) = Emulation::new(program).run_with_loop_detection();
    if has_loop {
        Some(registers.acc)
    } else {
        None
    }
}

fn find_acc_value_in_correct_program(program: &mut Program) -> Option<i32> {
    let length = program.len();

    for i in 0..length {
        program[i] = program[i].try_to_fix();

        let (has_loop, registers) = Emulation::new(program).run_with_loop_detection();
        if !has_loop && registers.ip as usize == length {
            return Some(registers.acc);
        }

        program[i] = program[i].try_to_fix();
    }

    None
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, program_text: &str) -> Result<String, ChallengeErr> {
        let program = parse_program(&program_text);

        find_acc_value_before_entering_loop(&program)
            .map(|v| Ok(v.to_string()))
            .unwrap_or(Err(ChallengeErr {}))
    }

    fn second_part(&self, program_text: &str) -> Result<String, ChallengeErr> {
        let mut program = parse_program(&program_text);

        find_acc_value_in_correct_program(&mut program)
            .map(|v| Ok(v.to_string()))
            .unwrap_or(Err(ChallengeErr {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_find_acc_value_before_entering_loop() {
        let test_program_text = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let program = parse_program(&test_program_text);

        assert_eq!(find_acc_value_before_entering_loop(&program), Some(5));
    }

    #[test]
    fn example_find_acc_value_in_correct_program() {
        let test_program_text = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let mut program = parse_program(&test_program_text);

        assert_eq!(find_acc_value_in_correct_program(&mut program), Some(8));
    }
}
