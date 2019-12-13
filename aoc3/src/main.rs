use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_input() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        if let Ok(l) = line {
            let v:Vec<&str> = l.split(',').collect();
            println!("{:?}", v);
        }
    }
}

enum WireSection {
    Vertical(i64, i64, i64, bool),
    Horizontal(i64, i64, i64, bool)
}

impl WireSection {
    fn new(x:i64, y:i64, direction:char, distance:i64) -> Option<WireSection> {
        match direction {
            'U' =>
                Some(WireSection::Vertical(x, y, y+distance, true)),
            'D' =>
                Some(WireSection::Vertical(x, y-distance, y, false)),
            'R' =>
                Some(WireSection::Vertical(x, y, x+distance, true)),
            'L' =>
                Some(WireSection::Vertical(x-distance, y, x, false)),
            _ =>
                None
        }
    }

    fn intersection(&self, wire: &WireSection) -> Option<(i64, i64)> {
        match (self, wire) {
            (WireSection::Vertical(xv, yv, yv2, _), WireSection::Horizontal(xh, yh, xh2, _))
            | (WireSection::Horizontal(xh, yh, xh2, _), WireSection::Vertical(xv, yv, yv2, _)) =>
                if ((yh > yv) && (yh < yv2))
                && ((xv > xh) && (xv < xh2)) {
                    Some((*xv, *yh))
                } else {
                    None
                }
            _ => None
        }
    }
}

fn main() {
    load_input();
    println!("Day 3 Stub");
}
