use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[path = "./dates.rs"]
mod dates;

pub fn penultimate_day_of_month(d: NaiveDate, day: Weekday) -> NaiveDate {
    // Implementation stolen from PenultimateDays.jl:
    // https://github.com/jakewilliami/PenultimateDays.jl/blob/2e3232f5dcf768ca8a1b2a9618a006b839411090/src/PenultimateDays.jl#L43-L46
    let i = day.number_from_monday() as i64;
    let ld_date = dates::last_day_of_month(d);
    let ld_i = ld_date.weekday().number_from_monday() as i64;
    let ld_day = 7 * ((ld_i < i) as i64) - i + ld_i + 1;
    ld_date
        .checked_sub_signed(Duration::days(ld_day - 1))
        .unwrap()
        .checked_sub_signed(Duration::weeks(1))
        .unwrap()
}
