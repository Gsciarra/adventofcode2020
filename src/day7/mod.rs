use crate::utils::{get_input, get_input_by_lines, print_solution};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solution1() {
    let input: Vec<Rule> = get_input_by_lines("input-day7.txt").unwrap();
    let mut bag_hierarchy = BagHierarchy::new();

    for rule in input.iter() {
        let bags_contained_in = bag_hierarchy.add(rule);
        bag_hierarchy.update_contained(&bags_contained_in, &rule.name);
    }

    // println!("{:#?}", bag_hierarchy);
    print_solution(
        7,
        1,
        bag_hierarchy.contained_by_count(&"shiny gold", &mut HashSet::new()),
    )
}

pub fn solution2() {
    let input: Vec<Rule> = get_input_by_lines("input-day7.txt").unwrap();
    let mut bag_hierarchy = BagHierarchy::new();

    for rule in input.iter() {
        let bags_contained_in = bag_hierarchy.add(rule);
        bag_hierarchy.update_contained(&bags_contained_in, &rule.name);
    }

    print_solution(7, 2, bag_hierarchy.tot_contains(&"shiny gold"))
}

#[derive(Debug)]
struct Rule {
    name: String,
    contains: Vec<(usize, String)>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_contains: Vec<String> = s.split("contain").map(String::from).collect();
        let name_raw = name_contains[0].clone();
        let name = name_raw
            .trim()
            .split_whitespace()
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");

        let contains_raw: Vec<Vec<&str>> = name_contains[1]
            .trim()
            .split(",")
            .map(|s| s.trim().split_whitespace().collect())
            .collect();
        let mut contains = vec![];

        for contain_raw in contains_raw {
            match contain_raw[0].parse::<usize>() {
                Ok(tot_bags) => {
                    if tot_bags > 0 {
                        contains.push((tot_bags, format!("{} {}", contain_raw[1], contain_raw[2])))
                    }
                }
                Err(_) => (),
            };
        }
        Ok(Rule { name, contains })
    }
}

#[derive(Debug)]
struct BagTree {
    contains: HashSet<String>,
    contained: HashSet<String>,
    contains_count: HashMap<String, usize>,
}

impl BagTree {
    fn new() -> Self {
        BagTree {
            contains: HashSet::new(),
            contained: HashSet::new(),
            contains_count: HashMap::new(),
        }
    }

    fn add_contains(&mut self, contains: &Vec<(usize, String)>) {
        for (n, c) in contains {
            self.contains.insert(c.clone());
            self.contains_count.insert(c.clone(), *n);
        }
    }

    fn add_contained(&mut self, contained: &str) -> bool {
        self.contained.insert(String::from(contained))
    }
}

#[derive(Debug)]
struct BagHierarchy {
    map: HashMap<String, BagTree>,
}

impl BagHierarchy {
    fn new() -> Self {
        BagHierarchy {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, rule: &Rule) -> HashSet<String> {
        let bag = self
            .map
            .entry(rule.name.clone())
            .or_insert_with(|| BagTree::new());

        bag.add_contains(&rule.contains);
        bag.contains.clone()
    }

    fn update_contained(&mut self, bags: &HashSet<String>, contained_by: &str) {
        for b in bags {
            self.map
                .entry(b.clone())
                .or_insert_with(|| BagTree::new())
                .add_contained(contained_by);
        }
    }

    fn contained_by_count(&self, bag_name: &str, excluded: &mut HashSet<String>) -> usize {
        let bag = self.map.get(bag_name);
        excluded.insert(String::from(bag_name));
        match bag {
            None => 0,
            Some(bag_tree) => {
                let contained: HashSet<String> = bag_tree
                    .contained
                    .iter()
                    .filter(|&name| !excluded.contains(name))
                    .map(|s| s.clone())
                    .collect();
                contained.iter().fold(contained.len(), |tot, b| {
                    tot + self.contained_by_count(b, excluded)
                })
            }
        }
    }

    fn tot_contains(&self, bag_name: &str) -> usize {
        let bag = self.map.get(bag_name).unwrap();

        bag.contains.iter().fold(0, |tot, b_name| {
            tot + bag.contains_count.get(b_name).unwrap() + bag.contains_count.get(b_name).unwrap() * self.tot_contains(b_name)
        })
    }
}
