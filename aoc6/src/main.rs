use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

#[derive(Debug)]
struct Planet {
    orbited: Option<String>,
    orbiting: Vec<String>
}

#[derive(Debug)]
struct SolarSystem {
    planets: HashMap<String, Planet>
}

impl SolarSystem {
    fn new() -> Self {
        SolarSystem { planets: HashMap::new() }
    }

    fn add(&mut self, planet1:String, planet2:String) {
        match self.planets.get_mut(&planet1) {
            Some(p) => {
                p.orbiting.push(planet2.to_owned());
            }
            None => {
                self.planets.insert(
                    planet1.to_owned(),
                    Planet { orbited: None, orbiting: vec![planet2.to_owned()]}
                );
            }
        }

        match self.planets.get_mut(&planet2) {
            Some(p) => {
                p.orbited = Some(planet1.to_owned());
            }
            None => {
                self.planets.insert(
                    planet2.to_owned(),
                    Planet { orbited: Some(planet1.to_owned()), orbiting: Vec::new() }
                );
            }
        }
    }
}

fn process_planets(planets: Vec<(String, String)>) -> SolarSystem {
    let mut system = SolarSystem::new();
    for (planet1, planet2) in planets {
        system.add(planet1, planet2)
    }
    system
}

fn load_file() -> Vec<(String, String)> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader
        .lines()
        .filter_map({ |x| 
            match x {
                Ok(a) => {
                    let v:Vec<&str> = a.split(")").collect();
                    Some((v[0].to_owned(), v[1].to_owned()))
                },
                _ => None
            }
         })
        .collect()
}

fn main() {
    println!("Hello, world!");
    let system = process_planets(load_file());
    dbg!(system);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbits_1() {
        let planets = vec![
            ("COM".to_owned(),"B".to_owned()),
            ("B".to_owned(),"C".to_owned()),
            ("C".to_owned(),"D".to_owned()),
            ("D".to_owned(),"E".to_owned()),
            ("E".to_owned(),"F".to_owned()),
            ("B".to_owned(),"G".to_owned()),
            ("G".to_owned(),"H".to_owned()),
            ("D".to_owned(),"I".to_owned()),
            ("E".to_owned(),"J".to_owned()),
            ("J".to_owned(),"K".to_owned()),
            ("K".to_owned(),"L".to_owned()),
            ];
        let system = process_planets(planets);
        assert_eq!(0,1);
    }

}