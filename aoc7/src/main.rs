use itertools::Itertools;
use intcode::IntComputer;

use std::fs::File;
use std::io::{Read, BufReader};

fn load_input() -> Vec<i64> {
    let file = File::open("input.txt").unwrap();
    let mut input = String::new();
    BufReader::new(file).read_to_string(&mut input).unwrap();
    input.split(',')
        .filter_map({ |x| x.parse().ok() })
        .collect()
}

fn phase_permutations(phasevec:Vec<i64>) -> Vec<Vec<i64>> {
    let len = phasevec.len();
    phasevec.into_iter().permutations(len).collect()
}

fn run_phase(program: &Vec<i64>, phase: i64, input: i64) -> i64 {
    let mut ic = IntComputer::load(program.clone());
    ic.write(phase);
    ic.write(input);
    ic.execute();
    ic.read().unwrap()
}

fn run_amplifiers(program: &Vec<i64>, phases:Vec<i64>) -> i64 {
    let mut output = 0;
    for phase in phases {
        output = run_phase(program, phase, output);
    };
    output
}

fn find_max_thrust_phase(program: &Vec<i64>) -> i64 {
    let mut max_thrust = 0;
    for phase in phase_permutations(vec![0,1,2,3,4]) {
        let output = run_amplifiers(program, phase);
        if output > max_thrust {
            max_thrust = output
        }
    }
    max_thrust
}

fn main() {
    let program = load_input();
    let max_thrust = find_max_thrust_phase(&program);
    println!("{}", max_thrust);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let output = run_amplifiers(&program, vec![4,3,2,1,0]);
        assert_eq!(output, 43210);

        let program = vec![
            3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,
            1,24,23,23,4,23,99,0,0
            ];
        let output = run_amplifiers(&program, vec![0,1,2,3,4]);
        assert_eq!(output, 54321);

        let program = vec![
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
            ];
        let output = run_amplifiers(&program, vec![1,0,4,3,2]);
        assert_eq!(output, 65210);
    }
}
