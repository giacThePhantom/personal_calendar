use std::fs;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;

struct Event {
    name: String,
    starting_date: String,
    ending_date: String,
    depends_on: String,
    repeat: String,
    tag: String,
    location: String,
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = match read_lines(&args[1]) {
        Ok(res) => res,
        Err(_) => panic!("ERROR")
    };
    for line in contents {
        let temp = line.unwrap();
        match temp.chars().nth(0) {
            None => println!("None"),
            Some('>') => println!("Found event"),
            Some('#') => println!("Found property"),
            _ => ()
        }

        //if temp.chars().nth(0) == Some('#') {
        //    println!("{}", temp);
        //}
    }


}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
