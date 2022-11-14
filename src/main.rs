use chrono::{Local, Weekday};
use clap::{crate_authors, crate_version, Parser};

mod penultimate;

#[derive(Parser)]
#[command(name = "pt", author = crate_authors!("\n"), version = crate_version!())]
/// When is the next penultimate Tuesday of the month?
struct Cli {}

fn main() {
	let _cli = Cli::parse();
	
	let today = Local::now();
	let penultimate_tuesday = penultimate::penultimate_day_of_month(today, Weekday::Tue);
	let penultimate_tuesday_str = penultimate_tuesday.format("%A, %e %B, %Y");
	println!("{penultimate_tuesday_str}");
}

