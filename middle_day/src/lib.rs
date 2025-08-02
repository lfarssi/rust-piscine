use chrono::{NaiveDate,Weekday,Datelike};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let leap_year= year%4==0 && (year%100!=0 || year%400==0);
    let days_in_year=if leap_year{366}else{365};
    if days_in_year%2==0{
        None
    } else {
         let target  = NaiveDate::from_yo_opt(year as i32,days_in_year)?;
         Some(target.weekday())
    }

}