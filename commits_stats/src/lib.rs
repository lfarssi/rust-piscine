use std::collections::HashMap;
use json::JsonValue;
use chrono::{DateTime, Utc, Datelike};

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *map.entry(login.to_string()).or_insert(0) += 1;
        }
    }

    map
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let date_utc: DateTime<Utc> = date.with_timezone(&Utc);
                let iso_week = date_utc.iso_week();
                let key = format!("{}-W{}", iso_week.year(), iso_week.week());
                *map.entry(key).or_insert(0) += 1;
            }
        }
    }

    map
}