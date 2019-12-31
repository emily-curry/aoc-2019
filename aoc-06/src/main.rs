use std::collections::HashMap;
use std::fs;

const ME: &str = "YOU";
const SAN: &str = "SAN";

fn main() {
    let input = fs::read_to_string("aoc-06/input.txt").unwrap();
    let map = OrbitMap::from(&input);

    let chksum = map.checksum();
    println!("Checksum: {}", chksum);

    let chain = map.find_path(ME, SAN);
    // Exclude ME and SAN, and subtract 1 hop as the starting loc.
    println!("Traversal chain length: {:?}", chain.len() - 3);
}

struct OrbitMap<'a> {
    objects: HashMap<&'a str, Orbiter<'a>>,
}

impl<'a> OrbitMap<'a> {
    pub fn from(input: &'a str) -> OrbitMap<'a> {
        let mut map = OrbitMap {
            objects: HashMap::new(),
        };

        let lines = input.split("\n");
        for line in lines {
            let objs: Vec<&str> = line.split(")").collect();
            assert_eq!(objs.len(), 2);
            let parent = objs[0];
            let child = objs[1];
            map.insert(parent, child);
        }

        map
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
            result += self.get_chain(key).len() as i32 - 1;
        }
        result
    }

    pub fn find_path(&self, start: &'a str, end: &'a str) -> Vec<&'a str> {
        let start_chain = self.get_chain(start);
        let end_chain = self.get_chain(end);
        let (item, start_idx, end_idx) = self.first_common_object(&start_chain, &end_chain);
        let mut chain: Vec<&'a str> = Vec::new();
        chain.extend(start_chain.split_at(start_idx).0.iter().map(|x| *x));
        chain.push(item);
        chain.extend(end_chain.split_at(end_idx).0.iter().rev().map(|x| *x));
        chain
    }

    fn first_common_object(
        &self,
        start: &Vec<&'a str>,
        end: &Vec<&'a str>,
    ) -> (&'a str, usize, usize) {
        for (index, item) in start.iter().enumerate() {
            if let Some(i) = end.iter().position(|x| x == item) {
                return (item, index, i);
            }
        }
        panic!("No common objects");
    }

    fn get_object(&mut self, key: &'a str) -> &mut Orbiter<'a> {
        if !self.objects.contains_key(key) {
            self.objects.insert(key, Orbiter::new());
        }
        self.objects.get_mut(key).unwrap()
    }

    fn get_chain(&self, key: &'a str) -> Vec<&'a str> {
        match self.objects.get(key).unwrap().parent {
            None => vec![key],
            Some(i) => {
                let mut y = vec![key];
                y.append(&mut self.get_chain(i));
                y
            }
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
