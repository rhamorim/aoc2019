// fn main() {
//     println!("Hello, world!");
// }

use std::io::{BufRead, BufReader};
use std::fs::File;

fn calc_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let fuel: u64 = lines
        .filter_map({|x|
            x.ok()?.parse().ok()
        })
        .map({|mass| calc_fuel(mass)})
        .sum();
    println!("{}", fuel);
    Ok(())
}