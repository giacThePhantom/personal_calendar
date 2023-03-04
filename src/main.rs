use chrono::{NaiveDate, Weekday};
use inquire::{Text, validator::{StringValidator, Validation}, DateSelect};


fn main() {
    let validator = |input: &str| if input.chars().count() > 140 {
        Ok(Validation::Invalid("You're only allowed 140 characters.".into()))
    } else {
        Ok(Validation::Valid)
    };

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .prompt();

    match status {
        Ok(status) => println!("Your status is being published..."),
        Err(err) => println!("Error while publishing your status: {}", err),
    };

    let date = DateSelect::new("When do you want to travel?")
        .with_starting_date(NaiveDate::from_ymd(2021, 8, 1))
        .with_min_date(NaiveDate::from_ymd(2021, 8, 1))
        .with_max_date(NaiveDate::from_ymd(2021, 12, 31))
        .with_week_start(Weekday::Mon)
        .with_help_message("Possible flights will be displayed according to the selected date").with_vim_mode(true)
        .prompt();

    match date {
        Ok(date) => println!("No flights available for this date. {:?}", date),
        Err(_) => println!("There was an error in the system."),
    }

}
