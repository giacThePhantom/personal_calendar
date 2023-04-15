use std::env;


mod event;
mod read_data;



fn main() {
    let args: Vec<String> = env::args().collect();
    let events = read_data::get_events(&args[1]);
    dbg!(events);
}
