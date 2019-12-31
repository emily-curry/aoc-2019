use std::collections::HashMap;
use std::fs;

const ME: &str = "YOU";
const SAN: &str = "SAN";

fn main() {
    let input = fs::read_to_string("aoc-06/input.txt").unwrap();
    let lines = input.split("\n");
    let mut map = OrbitMap::new();
    for line in lines {
        let objs: Vec<&str> = line.split(")").collect();
        assert_eq!(objs.len(), 2);
        let parent = objs[0];
        let child = objs[1];
        map.insert(parent, child);
    }

    let chksum = map.checksum();
    println!("Checksum: {}", chksum);

    let chain = map.get_chain(ME);
    println!("Checksum: {:?}", chain);
}

struct OrbitMap<'a> {
    objects: HashMap<&'a str, Orbiter<'a>>,
}

impl<'a> OrbitMap<'a> {
    pub fn new() -> OrbitMap<'a> {
        OrbitMap {
            objects: HashMap::new(),
        }
    }

    pub fn insert(&mut self, parent: &'a str, child: &'a str) {
        let parent_orbiter = self.get_object(parent);
        parent_orbiter.children.push(child);
        let mut child_orbiter = self.get_object(child);
        child_orbiter.parent = Some(parent);
    }

    pub fn checksum(&self) -> i32 {
        let mut result = 0;
        for (key, _) in self.objects.iter() {
            result += self.get_orbits(key);
        }
        result
    }

    fn get_object(&mut self, key: &'a str) -> &mut Orbiter<'a> {
        if !self.objects.contains_key(key) {
            self.objects.insert(key, Orbiter::new());
        }
        self.objects.get_mut(key).unwrap()
    }

    pub fn get_chain(&self, key: &'a str) -> Vec<&'a str> {
        match self.objects.get(key).unwrap().parent {
            None => vec![key],
            Some(i) => {
                let mut y = vec![key];
                y.append(&mut self.get_chain(i));
                y
            }
        }
    }

    fn get_orbits(&self, key: &'a str) -> i32 {
        match self.objects.get(key).unwrap().parent {
            None => 0,
            Some(i) => self.get_orbits(i) + 1,
        }
    }
}

struct Orbiter<'a> {
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}

impl<'a> Orbiter<'a> {
    fn new() -> Orbiter<'a> {
        Orbiter {
            parent: None,
            children: Vec::new(),
        }
    }
}
