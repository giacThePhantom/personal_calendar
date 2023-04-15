use std::env;
use std::io::{self, Write};
use termion::{color, style, terminal_size};


mod event;
mod read_data;



fn main() {
    let args: Vec<String> = env::args().collect();
    let events = read_data::get_events(&args[1]);
    dbg!(events);
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic", style::Italic);
    let (n_col, _) = terminal_size().unwrap();
    for i in 0..n_col {
        print!("{} ", color::Bg(color::Red));
    }
    io::stdout().flush().unwrap();
}
