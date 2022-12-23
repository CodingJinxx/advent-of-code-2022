/*
The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot.
As your boats approach land, the Elves begin taking inventory of their supplies.
One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line.
Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:
 */

use std::fs;

fn main() {
    let mut sorted_elves = audit_elves(fs::read_to_string("src/day_one/input").expect("Unable to read file").as_str());
    sorted_elves.sort_by(|x, y| y.calories.partial_cmp(&x.calories).expect("Unable to sort"));
    println!("Max Cals of top elf: {}", sorted_elves.first().expect("Cant take top elf").calories);
    println!("Max Cals of top three combined: {}", sorted_elves.iter().take(3).map(|x| x.calories).sum::<i32>());
}

fn audit_elves(lines: &str) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf = Elf::new();
    for line in lines.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = Elf::new();
        } else {
            elf.add_item(line);
        }
    }
    elves.push(elf);
    elves
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: i32
}

impl Elf {
    fn add_item(&mut self, item: &str) {
        self.calories += item.parse::<i32>().unwrap();
    }

    fn new() -> Elf {
        Elf {
            calories: 0
        }
    }
}
