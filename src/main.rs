use chrono::{Local, Weekday};
use clap::{crate_authors, crate_version, Parser};

mod penultimate;

#[derive(Parser)]
#[command(name = "pt", author = crate_authors!("\n"), version = crate_version!())]
/// When is the next penultimate Tuesday of the month?
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    display_penultimate_day(Weekday::Tue);
}

fn display_penultimate_day(wd: Weekday) {
    let today = Local::now().date_naive();
    let penultimate_tuesday = penultimate::penultimate_day_of_month(today, wd);
    let date_format_str = "%A, %e %B, %Y";
    
    if penultimate_tuesday == today {
        let human_readable_weekday = penultimate_tuesday.format("%A");
        println!("Today is the penultimate {} of the month!", human_readable_weekday);
        
        let penultimate_tuesday = penultimate::penultimate_day_of_month(today, wd);
        let penultimate_tuesday_str = penultimate_tuesday.format(date_format_str);
        println!("The penultimate {} of next month is on {}", human_readable_weekday, penultimate_tuesday_str);
    } else {
        let penultimate_tuesday_str = penultimate_tuesday.format(date_format_str);
        println!("{penultimate_tuesday_str}");
    }
}
