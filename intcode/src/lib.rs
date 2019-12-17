use std::convert::TryInto;
use std::collections::VecDeque;

pub struct IntComputer {
    state: Vec<i64>,
    position: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl IntComputer {
    pub fn execute(&mut self) {
        self.position = 0;
        while self.position <= self.state.len() {
            let (opcode, mode1, mode2, _mode3) = self.opcode();
            match opcode {
                1 => {
                    let arg1 = self.argument_value(mode1, 1);
                    let arg2 = self.argument_value(mode2, 2);
                    let arg3:usize = self.argument_value(1, 3).try_into().unwrap();
                    self.state[arg3] = arg1 + arg2;
                    self.position += 4;
                },
                2 => {
                    let arg1 = self.argument_value(mode1, 1);
                    let arg2 = self.argument_value(mode2, 2);
                    let arg3:usize = self.argument_value(1, 3).try_into().unwrap();
                    self.state[arg3] = arg1 * arg2;
                    self.position += 4;
                },
                3 => {
                    let arg1:usize = self.argument_value(1, 1).try_into().unwrap();
                    if let Some(i) = self.input.pop_front() {
                        self.state[arg1] = i
                    }
                    self.position += 2;
                },
                4 => {
                    let arg1 = self.argument_value(mode1, 1);
                    self.output.push_back(arg1);
                    self.position += 2;
                }
                99 => break,
                _ => ()
            }
        }
    }

    pub fn load(state: Vec<i64>) -> IntComputer {
        IntComputer {
            state : state,
            position : 0,
            input: VecDeque::new(),
            output: VecDeque::new()
        }
    }

    pub fn value(self, position: usize) -> i64 {
        self.state[position]
    }

    pub fn set(&mut self, position: usize, value: i64) {
        self.state[position] = value
    }

    pub fn write(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn read(&mut self) -> Option<i64>{
        self.output.pop_front()
    }

    fn opcode(&self) -> (i64, i64, i64, i64) {
        let raw_opcode = self.state[self.position];
        let (mode3, raw_opcode) = div_rem(raw_opcode, 10000);
        let (mode2, raw_opcode) = div_rem(raw_opcode, 1000);
        let (mode1, raw_opcode) = div_rem(raw_opcode, 100);
        (raw_opcode, mode1, mode2, mode3)
    }

    fn argument_value(&self, mode:i64, argument: usize) -> i64 {
        match mode {
            1 => {
                self.state[self.position + argument]
            },
            _ => {
                let position:usize = self.state[self.position + argument].try_into().unwrap();
                self.state[position]
            }
        }
    }
}

pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut ic = IntComputer::load(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        ic.execute();
        assert_eq!(ic.value(0), 3500);
    }

    #[test]
    fn test_opcode() {
        let ic = IntComputer::load(vec![1002,4,3,4,33]);
        assert_eq!(ic.opcode(), (2,0,1,0))
    }

    #[test]
    fn test_argmode() {
        let mut ic = IntComputer::load(vec![1002,4,3,4,33]);
        ic.execute();
        assert_eq!(ic.value(4), 99)
    }

    #[test]
    fn test_negative() {
        let mut ic = IntComputer::load(vec![1101,100,-1,4,0]);
        ic.execute();
        assert_eq!(ic.value(4), 99)
    }

    #[test]
    fn input_output() {
        let mut ic = IntComputer::load(vec![3,0,4,0,99]);
        ic.write(145);
        ic.execute();
        assert_eq!(ic.read(), Some(145));
        assert_eq!(ic.read(), None)
    }
}