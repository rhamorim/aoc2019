use itertools::Itertools;
use intcode::{ IntComputer, ExecutionState };

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

fn run_amplifiers_feedback(program: &Vec<i64>, phases:Vec<i64>) -> i64 {
    let mut amplifiers = Vec::new();
    for phase in phases {
        let mut ic = IntComputer::load(program.clone());
        ic.write(phase);
        amplifiers.push(ic);
    }
    let mut cur = 0;
    amplifiers.get_mut(cur).unwrap().write(0);
    amplifiers.get_mut(cur).unwrap().execute();
    let amps = amplifiers.len();
    loop {
        let amp_p = cur % amps;
        let amp_n = (cur+1) % amps;
        while let Some(o) = amplifiers.get_mut(amp_p).unwrap().read() {
            amplifiers.get_mut(amp_n).unwrap().write(o)
        }
        cur += 1;
        let state = amplifiers.get_mut(amp_n).unwrap().execute();
        match state {
            ExecutionState::Halted if (amp_n+1) == amps => break,
            _ => continue
        }
    }
    amplifiers.get_mut(cur%amps).unwrap().read().unwrap()
}

fn find_max_thrust_phase_feedback(program: &Vec<i64>) -> i64 {
    let mut max_thrust = 0;
    for phase in phase_permutations(vec![5,6,7,8,9]) {
        let output = run_amplifiers_feedback(program, phase);
        if output > max_thrust {
            max_thrust = output
        }
    }
    max_thrust
}

fn main() {
    let program = load_input();
    let max_thrust = find_max_thrust_phase(&program);
    let max_feedback = find_max_thrust_phase_feedback(&program);
    println!("{}, {}", max_thrust, max_feedback);
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

    #[test]
    fn test_examples_part_2() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let output = run_amplifiers_feedback(&program, vec![9,8,7,6,5]);
        assert_eq!(output, 139629729);

        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let output = run_amplifiers_feedback(&program, vec![9,7,8,5,6]);
        assert_eq!(output, 18216);
    }
}
