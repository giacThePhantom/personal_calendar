use std::fs;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;
use itertools::{Itertools, Position};

enum EventData {
    Custom(&'static str),
}

#[derive(Debug)]
struct Event {
    name: Option<String>,
    starting_date: Option<String>,
    ending_date: Option<String>,
    depends_on: Option<String>,
    repeat: Option<String>,
    tag: Option<String>,
    location: Option<String>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name : None,
            starting_date: None,
            ending_date: None,
            depends_on: None,
            repeat: None,
            tag: None,
            location: None,
        }
    }
}


impl Event {
    fn new(data : & Vec<String>) -> Self{

        println!("Building new event");
        dbg!(data);
        let mut res = Event::default();
        let mut name = String::new();
        let mut property = String::new();
        for i in data{
            match i.chars().nth(0) {
                None => (),
                Some('>') => {
                    name.push_str(&i[1..].trim());
                    // res.name = Some(i.clone());
                },
                Some('#') => {
                    if property.len() > 0{
                        dbg!(&property);
                    }
                        property.clear();
                    property.push_str(&i[1..].trim());
                }
                _ => {
                    if property.len() > 0 {
                        property.push_str("\n");
                        property.push_str(&i.trim());
                    }
                    else if name.len() > 0 {
                        name.push_str("\n");
                        name.push_str(&i.trim());
                    }

                    println!("FOUND CONTINUATION OF PROPERTY");
                }
            }
        }
        res.name = Some(name);
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
