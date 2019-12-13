use std::fs::File;
use std::io::{Read, BufReader};

fn load_input() -> Vec<usize> {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();
    BufReader::new(file).read_to_string(&mut input).unwrap();
    input.split(',')
        .filter_map({ |x| x.parse().ok() })
        .collect()
}

struct IntComputer {
    state: Vec<usize>,
    position: usize,
}

impl IntComputer {
    fn execute(&mut self) {
        self.position = 0;
        while self.position <= self.state.len() {
            let opcode = self.state[self.position];
            let pos_1  = self.state[self.position+1];
            let pos_2  = self.state[self.position+2];
            let pos_t  = self.state[self.position+3];
            match opcode {
                1 => {
                    self.state[pos_t] = self.state[pos_1] + self.state[pos_2];
                    self.position += 4;
                },
                2 => {
                    self.state[pos_t] = self.state[pos_1] * self.state[pos_2];
                    self.position += 4;
                },
                99 => break,
                _ => ()
            }
        }
    }

    fn load(state: Vec<usize>) -> IntComputer {
        IntComputer { state : state, position : 0 }
    }

    fn load_from_file() -> IntComputer {
        IntComputer::load(load_input())
    }

    fn value(self, position: usize) -> usize {
        self.state[position]
    }

    fn set(&mut self, position: usize, value: usize) {
        self.state[position] = value
    }
}

fn part1() {
    let mut ic = IntComputer::load_from_file();
    ic.set(1,12);
    ic.set(2,2);
    ic.execute();
    println!("{}", ic.value(0));
}

fn part2() {
    let program = load_input();
    for noun in 0..99 {
        for verb in 0..99 {
            let mut ic = IntComputer::load(program.clone());
            ic.set(1, noun);
            ic.set(2, verb);
            ic.execute();
            if ic.value(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let mut ic = IntComputer::load(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        ic.execute();
        assert_eq!(ic.value(0), 3500);
    }

}