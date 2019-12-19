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

    fn orbits(&self) -> usize {
        let baseplanets = self.planets.iter()
            .filter_map(|(k, v)| if v.orbited == None { Some(k) } else { None } );

        let mut orbits = 0;

        for baseplanet in baseplanets {
            let mut v = vec![baseplanet.to_owned()];
            let mut base_n = 0;
            while v.len() > 0 {
                base_n += 1;
                v = self.orbiting(&v);
                orbits += base_n * v.len();
            }
        }

        orbits
    }

    fn orbiting(&self, planetkeys:&Vec<String>) -> Vec<String> {
        planetkeys.iter()
            .map(|key| {
                self.planets.get(key).unwrap().orbiting.to_owned()
            })
            .flatten()
            .collect()
    }

    fn transfers(&self, key1:&String, key2:&String) -> usize {
        let path1 = self.path(key1);
        let path2 = self.path(key2);
        //let mut iter = path1.iter().zip(path2.iter());
        dbg!(path1, path2);
        0
    }

    fn path(&self, key:&String) -> Vec<String> {
        let mut v = Vec::new();
        let mut p = self.planets.get(key).unwrap();
        while let Some(o) = &p.orbited {
            v.push(o.to_owned());
            p = self.planets.get(o).unwrap();
        }
        v.reverse();
        v
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
    let you = "YOU".to_owned();
    let san = "SAN".to_owned();
    println!("{}, {}", system.orbits(), system.transfers(&you, &san));
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
        assert_eq!(system.orbits(),42);
    }

    #[test]
    fn test_transfers_1() {
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
            ("K".to_owned(),"YOU".to_owned()),
            ("I".to_owned(),"SAN".to_owned()),
            ];
        let system = process_planets(planets);
        let transfers = system.transfers(&"YOU".to_owned(), &"SAN".to_owned());
        assert_eq!(transfers,4);
    }

}