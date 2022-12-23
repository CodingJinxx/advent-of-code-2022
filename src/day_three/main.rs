extern crate core;

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Item(char);

impl Item {
    fn get_priority(&self) -> Option<i32>{
        if self.0 >= 'a' && self.0 <= 'z' {
            Some(self.0 as i32 - 96)
        } else if self.0 >= 'A' && self.0 <= 'Z' {
            Some(self.0 as i32 - 38)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    CompartmentA: Vec<Item>,
    CompartmentB: Vec<Item>
}

impl Rucksack {
    fn new() -> Rucksack {
        Rucksack {
            CompartmentA: Vec::new(),
            CompartmentB: Vec::new()
        }
    }

    fn get_shared_items(&self) -> Vec<&Item> {
        let mut shared_items : HashSet<&Item> = HashSet::new();
        for item in self.CompartmentA.iter() {
            if self.CompartmentB.contains(item) {
                shared_items.insert(item);
            }
        }
        shared_items.into_iter().collect()
    }

}


impl Into<Vec<Item>> for Rucksack {
    fn into(self) -> Vec<Item> {
        let mut items = self.CompartmentA;
        items.extend(self.CompartmentB);
        items
    }
}

trait ItemUnion {
    fn union(&self, other: &Self) -> Self;
}

impl ItemUnion for Vec<Item> {
    fn union(&self, other: &Self) -> Self {
        let mut shared_items : HashSet<&Item> = HashSet::new();
        for item in self.iter() {
            if other.contains(item) {
                shared_items.insert(item);
            }
        }
        shared_items.into_iter().map(|item| item.clone()).collect()
    }
}

struct ElfGroup(Vec<Item>, Vec<Item>, Vec<Item>);

impl ElfGroup {
    fn get_group_badge(&self) -> Option<Item> {
        let mut shared = self.0.union(&self.1).union(&self.2);
        shared.sort_by(|a, b| a.get_priority().unwrap().cmp(&b.get_priority().unwrap()));
        Some(shared.first().expect("No shared items").clone())
    }
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        let mut rucksack = Rucksack::new();
        for i in 0..(value.len() / 2) {
            rucksack.CompartmentA.push(Item(value.chars().nth(i).expect("Unable to get Item for Comp A")));
            rucksack.CompartmentB.push(Item(value.chars().nth(i + (value.len() / 2)).expect("Unable to get Item for Comp B")));
        }

        rucksack
    }
}

impl From<&Rucksack> for Vec<Item> {
    fn from(value: &Rucksack) -> Self {
        let mut items = value.CompartmentA.clone();
        items.extend(value.CompartmentB.clone());
        items
    }
}

fn main() {
    let input = fs::read_to_string("src/day_three/input").expect("Unable to read file");
    let input = input.lines();
    let mut rucksacks: Vec<Rucksack> = input.map(|x| Rucksack::from(x)).collect();
    let mut tally = 0;
    for rucksack in &rucksacks {
        let shared = rucksack.get_shared_items();
        println!("{:?}  {:?}", shared, shared.iter().map(|x| x.get_priority().expect("Unable to get prio")).collect::<Vec<i32>>());
        tally += shared.iter().map(|x| x.get_priority().expect("Unable to get prio")).sum::<i32>();
    }

    println!("Final Tally: {}", tally);

    let mut iterator = rucksacks.iter();

    let mut tally = 0;

    for i in 0..(rucksacks.len() / 3) {
        match ElfGroup(iterator.next().expect("Unable to take first").into(), iterator.next().expect("Unable to take second").into(), iterator.next().expect("Unable to take third").into()).get_group_badge()  {
            Some(item) => {
                println!("{:?}", item);
                tally += item.get_priority().expect("Unable to get prio");
            },
            None => ()
        }
    }

    println!("GROUP BADGE Final Tally: {}", tally);

}