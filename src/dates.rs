// We have to define this ourselves, because chrono doesn't have these
// very simple functions yet:
// https://github.com/chronotope/chrono/issues/722
// 
// There is also this crate:
// https://github.com/liquidscorpio/chrono-utils
// But it appears to be abandoned
// 
// Update: since writing these, I have found this crate:
// https://github.com/Kilerd/now
// However, it is probably best that I have implemented the only portion
// of this logic I need

use chrono::{Datelike, NaiveDate};


static DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];


fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
}

fn days_in_month(y: i32, m: u32) -> u32 {
    (DAYS_IN_MONTH[m as usize - 1] as u32) + ((m == 2 && is_leap_year(y)) as u32)
}

pub fn last_day_of_month(dt: NaiveDate) -> NaiveDate {
    let (m, y) = (dt.month(), dt.year());
    NaiveDate::from_ymd_opt(y, m, days_in_month(y, m)).unwrap()
}

