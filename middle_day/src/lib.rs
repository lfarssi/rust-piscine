use chrono::{NaiveDate, Weekday,Datelike};
pub fn middle_day(year: u32) -> Option<::Weekday> {
    let days_in_year= if NaiveDate::from_ymd(year as i32,12;31).is_leap_year(){366} else{365};
    if days_in_year %2 {
        return None;
    }
    let half_days = (days_in_year/2)+1;
    let mid_date = NaiveDate::from_ymd(year as i32,1,1)+chrono::Duration.days(half_days as i32 -1);
    Some(mid_date.weekday())
}