use std::fmt::format;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{ops::Add, vec};

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "habit.json";

type D = DateTime<Utc>;

/// Habit - Represents one Habit
/// label is the name of the habit
/// done_dates are the dates on which the Habit is marked as done
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Habit {
    pub label: String,
    pub done_dates: Vec<String>,
}

impl Habit {
    pub fn check_task(&mut self, date: String) {
        match self.done_dates.iter().position(|x| *x == date) {
            Some(i) => {
                self.done_dates.remove(i);
            }
            None => self.done_dates.push(date),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct HabitTracker {
    pub start_date: D,
    pub habits: Vec<Habit>,
}

impl HabitTracker {
    // Get the next week for the table
    pub fn next_week(&mut self) {
        let start_date = self.start_date;
        self.start_date = start_date.add(Duration::days(7));
    }

    // Get the previous week on the table
    pub fn previous_week(&mut self) {
        let start_date = self.start_date;
        self.start_date = start_date.checked_sub_signed(Duration::days(7)).unwrap();
    }

    // Get the date range
    pub fn get_date_range(&self) -> Vec<D> {
        let dates: &mut Vec<D> = &mut vec![];
        dates.push(self.start_date);
        for i in 1..7 {
            dates.push(self.start_date.add(Duration::days(i)));
        }
        dates.to_owned()
    }

    // Get the table header labels
    pub fn get_header_labels(&self) -> Vec<String> {
        self.get_date_range()
            .iter()
            // Format the header string with format()
            .map(|d| format(format_args!("{: ^3}", d.day())).to_string())
            .collect()
    }

    // This should send a matrix of bools, which should contain
    pub fn values(&self) -> Vec<Vec<bool>> {
        let date_range = self.get_date_range();
        let mut values: Vec<Vec<bool>> = vec![vec![false; date_range.len()]; self.habits.len()];
        for (i, habit) in self.habits.iter().enumerate() {
            for (j, date) in date_range.iter().enumerate() {
                if habit.done_dates.contains(&date.to_string()) {
                    values[i][j] = true;
                }
            }
        }
        values
    }

    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];
        for habit in self.habits.clone() {
            labels.push(habit.label)
        }
        labels
    }

    fn week_bounds(week: u32) -> D {
        let offset = chrono::offset::Local::now();
        let current_year = offset.year();
        let mon = NaiveDate::from_isoywd(current_year, week, Weekday::Mon);
        DateTime::<Utc>::from_utc(mon.and_hms(0, 0, 0), Utc)
    }

    pub fn store_state(&self) {
        let dir = dirs::config_dir().unwrap();
        let mut file = File::create(dir.with_file_name(FILE_NAME)).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())
            .unwrap();
    }

    pub fn fetch_state() -> Self {
        let dir = dirs::config_dir().unwrap();
        let path = dir.with_file_name(FILE_NAME);
        let data = fs::read_to_string(path);
        match data {
            Ok(s) => serde_json::from_str(s.as_str()).unwrap(),
            Err(_) => HabitTracker::default(),
        }
    }

    pub fn default() -> Self {
        HabitTracker {
            start_date: HabitTracker::week_bounds(Utc::now().iso_week().week()),
            habits: vec![],
        }
    }
}
