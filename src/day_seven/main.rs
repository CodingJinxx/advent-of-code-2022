#![deny(elided_lifetimes_in_paths)]

use std::fmt::Error;
use std::fs;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node<'a> {
    Directory {
        name: &'a str,
        contents: Vec<Node<'a>>
    },
    File {
        name: &'a str,
        size: i32
    }
}

#[derive(Debug)]
struct Computer<'a> {
    disk: FileSystem<'a>,
    current_directory: Path<'a>
}

#[derive(Debug)]
struct Path<'a> {
    path: Vec<&'a str>,
    index: usize
}

impl Path<'_> {
    fn new() -> Path<'static> {
        Path {
            path: Vec::new(),
            index: 0
        }
    }
    
    fn set_path(&mut self, path: &str) {
        self.path = path.split("/").collect();
    }
}

impl Iterator for Path<'_> {
    type Item = &'_ str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.len() == 0 {
            return None;
        }
        
        if self.index > self.path.len() {
            self.index = 0;
        } 
        else {
            self.index += 1;
        }

        let option = Some(self.path[self.index]);
        
        todo!();
    }
}

impl Computer<'_> {
    fn new() -> Computer<'static> {
         Computer {
            disk: FileSystem::new(),
            current_directory: Path::new()
        }
    }

    fn execute<'a>(&mut self, command: Command<'a>) {
        match command {
            Command::ChangeDirectory(name) => {
                println!("Changing directory to {}", name);
            },
            Command::ListDirectory {
                directories: Some(directories),
            } => {
                println!("Listing directories");
                for directory in directories {
                    println!(" - {}", directory);
                }
            },
            Command::ListDirectory {
                directories: None
            } => {
                println!("No Directories");
            },
        }
    }
}

#[derive(Debug)]
struct FileSystem<'a> {
    root: Node<'a>
}



impl FileSystem<'_> {
    fn new() -> FileSystem<'static> {
        FileSystem {
            root: Node::Directory {
                name: "root",
                contents: Vec::new()
            }
        }
    }

    fn path_valid(&self, path: &'_ Path<'_>) -> Result<bool, &str> {
        match &self.root {
            Node::Directory { name, contents }  => {
                if *name != "root" {
                    return Err("Filesystem root is formatted incorrectly");
                }

                todo!()
            },
            Node::File { name, size } => Err("Filesystem corrupt, root is a file")
        }
    }

    fn create_file(&mut self, path: &'_ Path<'_>) {

    }
}

#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    ListDirectory {
        directories: Option<Vec<String>>
    }
}

#[derive(Debug)]
struct LogEntry<'a> {
    command: &'a str,
    arguments: Vec<&'a str>,
    output: Option<String>
}

impl LogEntry<'_> {
    fn create_log<'a>(raw_log: &'a str) -> Vec<LogEntry<'a>> {
        let mut log_entries: Vec<LogEntry<'a>> = Vec::new();
        let logs : Vec<&str> = raw_log.split_once('$').expect("Invalid Log").1.split('$').collect();

        for entry in logs {
            let entry = entry.trim();
            let command : Vec<&str> = entry.split('\n').collect();

            let mut test = LogEntry {
                command: command[0].split(' ').collect::<Vec<&str>>()[0],
                arguments: match command[0].split_once(' ') {
                    Some((_, arguments)) => arguments.split(' ').collect(),
                    None => Vec::new()
                },
                output: match command.len() {
                    1 => None,
                    _ => {
                        let mut output = String::new();
                        for i in 1..command.len() {
                            output.push_str(command[i]);
                            output.push_str("\n");
                        }
                        Some(output)
                    }
                }
            };
            log_entries.push(test);
        }
        log_entries
    }
}
impl<'a> From<&LogEntry<'a>> for Command<'a> {
    fn from(input: & LogEntry<'a>) -> Self {
        match input.command {
            "cd" => Command::ChangeDirectory(input.arguments[0]),
            "ls" => Command::ListDirectory {
                directories: match &input.output {
                    Some(output) => {
                        let mut directories: Vec<String> = Vec::new();
                        for line in output.to_string().split('\n') {
                            if line.len() > 0{
                                directories.push(line.trim().to_string());
                            }
                        }
                        Some(directories)
                    },
                    None => None
                }
            },
            _ => panic!("Unknown command")
        }
    }
}

fn main() {
    let log   = std::fs::read_to_string("src/day_seven/input").expect("Unable to read file");
    let log = log.as_str();
    let mut computer = Computer::new();
    let log_entries = LogEntry::create_log(log);
    for entry in log_entries {
        computer.execute(Command::from(&entry));
    }
}