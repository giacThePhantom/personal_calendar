use std::collections::HashMap;

#[derive(Debug)]
pub struct Event {
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

impl Event {
    pub fn new(data : &Vec<String>) -> Self{
        let mut res = Event::default();
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
                            res.properties.insert(
                                i[1..].trim()
                                    .to_string(),
                                "".to_string()
                            );
                        }

                    }
                }
                _ => (),
            }
        }
        return res;

    }
}
