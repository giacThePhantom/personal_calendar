use std::fs;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use itertools::{Itertools, Position};
use std::collections::HashMap;


#[derive(Debug)]
struct Event {
    name: Option<String>,
    properties : HashMap<String, String>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name : None,
            properties : HashMap::new(),
        }
    }
}

fn merge_tags_one_line(data : &Vec<String>) -> Vec<String>{
    let mut res = Vec::<String>::new();
    for line in data {
        match line.chars().nth(0) {
            None => (),
            Some('>') | Some('#') => {
                res.push(line.to_string());
            },
            _ => {
                let last_idx = res.len() -1;
                res[last_idx].push_str("\n");
                res[last_idx].push_str(&line);
            },
        }
    }
    return res;
}

impl Event {
    fn new(data : &Vec<String>) -> Self{


        println!("Building new event");
        let mut res = Event::default();
        let data = merge_tags_one_line(data);
        dbg!(&data);
        for i in data{
            match i.chars().nth(0) {
                None => (),
                Some('>') => {
                    res.name = Some(i[1..]
                        .trim()
                        .to_string());
                },
                Some('#') => {
                    match i[1..].split_once(':') {
                        Some((key, value)) => {
                            res.properties.insert(
                                key.trim()
                                    .to_string(),
                                value.trim()
                                    .to_string()
                            );
                        }
                        None => {
                            println!("Didn't found the delimeter :");
                        }

                    }
                }
                _ => (),
            }
        }
        return res;

    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = match read_lines(&args[1]) {
        Ok(res) => res,
        Err(_) => panic!("Could not read configuration file")
    };
    let mut events = Vec::<Event>::new();
    let mut event_string = Vec::<String>::new();
    for line in contents.with_position() {
        match line {
            Position::Only(_) => {
                panic!("File event has only one line!");
            },
            Position::First(temp) | Position::Middle(temp) =>{
                let temp = temp.unwrap();
                match temp.chars().nth(0) {
                    None => (),
                    Some('>') => {
                        println!("FOUND >");
                        if ! event_string.is_empty() {
                            events.push(
                                Event::new(&event_string)
                            );
                            event_string.clear();
                        }
                        event_string.push(temp);
                    },
                    _ => {
                        event_string.push(temp);
                    }
                }
            },
            Position::Last(_) =>{
                events.push(Event::new(&event_string));

            }



        }

    }
    dbg!(events);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
