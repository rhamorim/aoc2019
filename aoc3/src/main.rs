use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

fn load_input() -> Vec<Wire> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
        .filter_map(
            |s|
            if let Ok(line) = s {
                Some(line_to_wire(&line))
             } else {
                None
             }
            )
        .collect()
}

fn str_to_wiresection(x:&mut i64, y:&mut i64, s:&str) -> Option<WireSection> {
    let (direction, sdistance) = s.split_at(1);
    let distance:i64 = sdistance.parse().ok()?;
    match direction {
        "U" =>
            {
                let old_y = *y;
                *y += distance;
                Some(WireSection::Vertical(*x, old_y, *y, true))
            },
        "D" =>
            {
                let old_y = *y;
                *y -= distance;
                Some(WireSection::Vertical(*x, *y, old_y, false))
            },
        "R" =>
            {
                let old_x = *x;
                *x += distance;
                Some(WireSection::Horizontal(old_x, *y, *x, true))
            },
        "L" =>
            {
                let old_x = *x;
                *x -= distance;
                Some(WireSection::Horizontal(*x, *y, old_x, false))
            },
            _ =>
            None
    }
}

fn str_to_wiresection_closure(x:i64, y:i64) -> impl FnMut(&str) -> Option<WireSection> {
    let mut x = x;
    let mut y = y;

    {move |s|
        str_to_wiresection(&mut x, &mut y, s)
    }
}

fn line_to_wire(line:&str) -> Wire {
    Wire(
        line
            .split(',')
            .filter_map(str_to_wiresection_closure(0, 0))
            .collect()
    )
}

#[derive(Debug)]
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
                Some(WireSection::Horizontal(x, y, x+distance, true)),
            'L' =>
                Some(WireSection::Horizontal(x-distance, y, x, false)),
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

impl fmt::Display for WireSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WireSection::Vertical(_, ymin, ymax, d) =>
                {
                    write!(f, "{}{}", if *d { "U" } else { "D" }, ymax-ymin)
                }
            WireSection::Horizontal(xmin, _, xmax, d) =>
                {
                    write!(f, "{}{}", if *d { "R" } else { "L" }, xmax-xmin)
                }
        }
    }
}

struct Wire(Vec<WireSection>);

impl Wire {
    fn intersections(&self, wire: &Wire) -> Vec<(i64,i64)> {
        let mut intersections:Vec<(i64,i64)> = Vec::new();
        for ws in self.0.iter() {
            for ws2 in wire.0.iter() {
                if let Some(i) = ws.intersection(ws2) {
                    intersections.push(i)
                }
            }
        }
        intersections
    }

    fn min_intersection_distance(&self, wire: &Wire) -> Option<i64> {
        self.intersections(wire).iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for w in self.0.iter() {
            if !first {
                write!(f, ",")?;
            } else {
                first = false;
            }
            write!(f, "{}", w)?;
        }
        Ok(())
    }
}

fn main() {
    let wires = load_input();
    let distance = wires[0].min_intersection_distance(&wires[1]);
    match distance {
        Some(d) => println!("{}", d),
        None => println!("No intersections")
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_line_to_wire_1() {
        let line = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire = line_to_wire(line);
        assert_eq!(line,wire.to_string());
    }

    #[test]
    fn test_intersection_1() {
        let wire1 = line_to_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = line_to_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        let minimum_distance = wire1.min_intersection_distance(&wire2);
        assert_eq!(minimum_distance,Some(159));
    }

    #[test]
    fn test_intersection_2() {
        let wire1 = line_to_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = line_to_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let minimum_distance = wire1.min_intersection_distance(&wire2);
        assert_eq!(minimum_distance,Some(135));
    }
}
