use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

enum JayNeedsTo {
    Lose,
    Win,
    Draw
}

#[derive(Debug, Clone, Copy)]
enum Player {
    Jay,
    Elf,
    Draw
}

impl Choice {
    fn based_on_elf(desired_game_result: JayNeedsTo, elf: Choice) -> Choice {
        match desired_game_result {
            JayNeedsTo::Lose => match elf {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper
            },
            JayNeedsTo::Win => match elf {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock
            },
            JayNeedsTo::Draw => elf
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RoundResult {
    winner: Player,
    choice : Choice
}

impl RoundResult {
    fn new(winner: Player, choice: Choice) -> RoundResult {
        RoundResult {
            winner,
            choice
        }
    }

    fn get_score(&self) -> i32 {
        let modifier = match self.choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        };

        modifier + match self.winner {
            Player::Jay => 6,
            Player::Elf => 0,
            Player::Draw => 3
        }
    }
}

fn main() {
    let mut game_processor = GameLogProcessor::new();
    let game_log = fs::read_to_string("src/day_two/input").expect("Unable to read file");
    let game_log = game_log.lines();
    for log in game_log {
        game_processor.process(log);
    }
    println!("{}", game_processor);
}

#[derive(Debug)]
struct GameLogProcessor {
    jay_score: i32,
    results: Vec<RoundResult>
}

impl Display for GameLogProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Results: ")?;
        for result in self.results.iter() {
            writeln!(f, "\t{:?}", result)?;
        }
        writeln!(f, "Jay's score: {}", self.jay_score)?;
        Ok(())
    }
}

impl Display for RoundResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Winner: {:?}, Choice: {:?}", self.winner, self.choice)
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Rock => write!(f, "Rock"),
            Choice::Paper => write!(f, "Paper"),
            Choice::Scissors => write!(f, "Scissors")
        }
    }
}

impl GameLogProcessor {
    fn new() -> GameLogProcessor {
        GameLogProcessor {
            jay_score: 0,
            results: vec![]
        }
    }

    fn process(&mut self, input: &str) -> RoundResult {
        let result : RoundResult = RoundResult::from(input); // Parse Input, Convert in Choices, Determine Results
        self.jay_score += result.get_score();
        self.results.push(result);
        result
    }
}

impl From<&str> for RoundResult {
    fn from(value: &str) -> Self {
        let (elf, human) = value.split_once(' ').expect("Unable to split");
        let elf = Choice::from(elf);
        let human = Choice::based_on_elf(JayNeedsTo::from(human), elf);

        match elf {
            Choice::Rock => match human {
                Choice::Rock => RoundResult::new(Player::Draw,Choice::Rock),
                Choice::Paper => RoundResult::new(Player::Jay, Choice::Paper),
                Choice::Scissors => RoundResult::new(Player::Elf, Choice::Scissors)
            },
            Choice::Paper => match human {
                Choice::Rock => RoundResult::new(Player::Elf, Choice::Rock),
                Choice::Paper => RoundResult::new(Player::Draw, Choice::Paper),
                Choice::Scissors => RoundResult::new(Player::Jay, Choice::Scissors)
            },
            Choice::Scissors => match human {
                Choice::Rock => RoundResult::new(Player::Jay, Choice::Rock),
                Choice::Paper => RoundResult::new(Player::Elf, Choice::Paper),
                Choice::Scissors => RoundResult::new(Player::Draw, Choice::Scissors)
            }
        }
    }
}

impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Unknown choice")
        }
    }
}

impl From<&str> for JayNeedsTo {
    fn from(value: &str) -> Self {
       match value {
            "X" => JayNeedsTo::Lose,
            "Y" => JayNeedsTo::Draw,
            "Z" => JayNeedsTo::Win,
           _ => panic!("Unknown choice")
       }
    }
}