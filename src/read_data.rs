use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::{Itertools, Position};

use crate::event::Event;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

pub fn get_events(filename: &str) -> Vec<Event> {
    let contents = match read_lines(filename) {
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
                            event_string = merge_tags_one_line(&event_string);
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
                event_string = merge_tags_one_line(&event_string);
                events.push(Event::new(&event_string));

            }



        }

    }
    return events;
}
