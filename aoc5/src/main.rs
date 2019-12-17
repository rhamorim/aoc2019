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

fn main() {
    let mut ic = IntComputer::load(load_input());
    ic.write(1);
    ic.execute();

    while let Some(i) = ic.read() {
        println!("{}", i)
    }
}
