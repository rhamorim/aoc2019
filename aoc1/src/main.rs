// fn main() {
//     println!("Hello, world!");
// }

use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel(mass: u64) -> u64 {
    let mut total = 0;
    let mut f = calc_fuel(mass);
    while f > 0 {
        total += f;
        f = calc_fuel(f);
    }
    total
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let masses: Vec<u64> = buf_reader
        .lines()
        .filter_map({ |x| x.ok()?.parse().ok() })
        .collect();
    let fuel: u64 = masses.iter().map({ |mass| calc_fuel(*mass) }).sum();
    let total_fuel: u64 = masses.iter().map({ |mass| total_fuel(*mass) }).sum();
    println!("{} - {}", fuel, total_fuel);
    Ok(())
}
