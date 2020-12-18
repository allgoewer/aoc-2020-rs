use super::{Day as DayTrait, Input};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<_> = s.split(' ').collect();

        match &instruction[..] {
            ["nop", decimal] => {
                let decimal = decimal.parse().or(Err("Invalid operand for NOP"))?;
                Ok(Instruction::Nop(decimal))
            }
            ["acc", decimal] => {
                let decimal = decimal.parse().or(Err("Invalid operand for ACC"))?;
                Ok(Instruction::Acc(decimal))
            }
            ["jmp", decimal] => {
                let decimal = decimal.parse().or(Err("Invalid operand for JMP"))?;
                Ok(Instruction::Jmp(decimal))
            }
            _ => Err("Invalid instruction"),
        }
    }
}

impl Instruction {
    fn switch_nop_jmp(&mut self) {
        let _ = std::mem::replace(
            self,
            match self {
                Self::Jmp(ref x) => Self::Nop(*x),
                Self::Nop(ref x) => Self::Jmp(*x),
                _ => *self,
            },
        );
    }

    fn terminates_with_accu(program: &[Instruction]) -> Option<i32> {
        let mut instruction_counter = HashSet::new();
        let mut accu = None;

        let mut process = Process::new(program);
        process.run_and(|p| {
            if instruction_counter.contains(&p.pc) {
                accu = Some(p.acc);
                false
            } else {
                instruction_counter.insert(p.pc);
                true
            }
        });

        accu
    }

    fn terminates_with_switched_instructions(program: &mut [Instruction]) -> i32 {
        let mut last_instruction = None;

        loop {
            // revert the last change to the program
            let last = if let Some(last) = last_instruction {
                program[last as usize].switch_nop_jmp();
                last + 1
            } else {
                0
            };

            for i in last..program.len() {
                match program[i] {
                    Instruction::Nop(_) | Instruction::Jmp(_) => {
                        program[i].switch_nop_jmp();
                        last_instruction = Some(i);
                        break;
                    }
                    _ => (),
                }
            }

            let mut instruction_counter = HashSet::new();

            let mut process = Process::new(&program);
            process.run_and(|p| {
                if instruction_counter.contains(&p.pc) {
                    false
                } else {
                    instruction_counter.insert(p.pc);
                    true
                }
            });

            if process.pc as usize >= program.len() {
                break process.acc;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Running,
    Stopped,
}

impl State {
    fn is_running(&self) -> bool {
        if let Self::Running = self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Process<'a> {
    program: &'a [Instruction],
    pc: isize,
    acc: i32,
}

impl<'a> Process<'a> {
    fn new(program: &'a [Instruction]) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) -> State {
        if let Some(instruction) = self.program.get(self.pc as usize) {
            match instruction {
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
                Instruction::Acc(val) => {
                    self.acc += val;
                    self.pc += 1;
                }
                Instruction::Jmp(val) => {
                    self.pc += val;
                }
            };
            State::Running
        } else {
            State::Stopped
        }
    }

    /// Run the program until the program counter is invalid.  
    /// f is called *before* every execution cycle.  
    /// If f returns false, the execution stops
    fn run_and<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Process) -> bool,
    {
        loop {
            if !f(self) {
                break;
            }

            if !self.step().is_running() {
                break;
            }
        }
    }
}

pub struct Day;

impl DayTrait for Day {
    fn part1(&self, input: &Input) -> String {
        let program: Vec<Instruction> = input
            .0
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        Instruction::terminates_with_accu(&program)
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &Input) -> String {
        let mut program: Vec<Instruction> = input
            .0
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        Instruction::terminates_with_switched_instructions(&mut program).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "nop +0
                           acc +1
                           jmp +4
                           acc +3
                           jmp -3
                           acc -99
                           acc +1
                           jmp -4
                           acc +6
                           ";

    const SAMPLE2: &str = "nop +0
                           acc +1
                           jmp +4
                           acc +3
                           jmp -3
                           acc -99
                           acc +1
                           jmp -4
                           acc +6
                           ";

    #[test]
    fn example_part1() {
        let program: Vec<Instruction> = SAMPLE1
            .lines()
            .map(str::trim_start)
            .filter_map(|line| line.parse().ok())
            .collect();

        assert_eq!(Instruction::terminates_with_accu(&program), Some(5));
    }

    #[test]
    fn example_part2() {
        let mut program: Vec<Instruction> = SAMPLE2
            .lines()
            .map(str::trim_start)
            .filter_map(|line| line.parse().ok())
            .collect();

        assert_eq!(
            Instruction::terminates_with_switched_instructions(&mut program),
            8
        );
    }
}
