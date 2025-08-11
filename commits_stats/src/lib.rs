use std::collections::HashMap;
// use json::JsonValue;
use chrono::{DateTime, Utc, Datelike};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map= HashMap::new();
    for member in data.members() {
        if let Some(login)= member["author"]["login"].as_str(){
            map.entry(String::from(login)).and_modify(|v| *v+=1).or_insert(0);
        }
    }
    map

}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for member in data.members(){
        if let Some(date)= member["commit"]["author"]["date"].as_str(){
            if let Ok(parsed_date) = DateTime::parse_from_rfc3339(date) {

               let date_utc = parsed_date.with_timezone(&Utc);
                let iso_week = date_utc.iso_week();
                let key = format!("{}-W{}", iso_week.year(), iso_week.week());
                map.entry(key).and_modify(|v| *v+=1).or_insert(0);
            }
        }
    }
    map
}