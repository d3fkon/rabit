use std::char;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::{self, create_dir_all, File};
use std::io::prelude::*;
use std::{ops::Add, vec};

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

// TODO: Change this?
const FILE_NAME: &str = "habit.json";

type D = DateTime<Utc>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum HabitType {
    BIT,
    COUNT,
    ALPHA,
}

/// Habit - Represents one Habit
/// label is the name of the habit
/// done_dates are the dates on which the Habit is marked as done
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub label: String,
    pub stats: HashMap<String, String>,
    pub done_dates: Vec<String>, // For backwards compatibility
    pub habit_type: HabitType,
}

impl Habit {
    pub fn check_task(&mut self, date: String, val: Option<char>) {
        // This may or may not exist
        let existing_stat = self.stats.get(&date);
        match self.habit_type {
            HabitType::BIT => match existing_stat {
                // Remove any existing stats for habit of type BIT
                Some(_) => {
                    self.stats.remove(&date);
                }
                // Add the stat if an existing stat doesn't exist
                None => {
                    self.stats.insert(date, String::from("true"));
                }
            },
            HabitType::COUNT => {
                self.stats.insert(
                    date.clone(),
                    if self.stats.contains_key(&date) {
                        (self.stats.get(&date).unwrap().parse::<i32>().unwrap() + 1).to_string()
                    } else {
                        String::from("0")
                    },
                );
            }
            HabitType::ALPHA => {
                self.stats.insert(date, val.unwrap().to_string());
            }
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

    // Return the right values based on the stats map
    pub fn values_v2(&self) -> Vec<Vec<Option<String>>> {
        let date_range = self.get_date_range();
        let mut values: Vec<Vec<Option<String>>> =
            vec![vec![None; date_range.len()]; self.habits.len()];

        for (i, habit) in self.habits.iter().enumerate() {
            for (j, date) in date_range.iter().enumerate() {
                if habit.stats.contains_key(&date.to_string()) {
                    let stat = habit.stats.get(&date.to_string()).unwrap();
                    // BUG: Fix this. Check for whitespaces and return None accordingly
                    values[i][j] = if true { Some(stat.to_string()) } else { None }
                }
            }
        }

        values
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

    // Get all the labels from the disk
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];
        for habit in self.habits.clone() {
            labels.push(habit.label)
        }
        labels
    }

    // Get the week bound for the current view
    fn week_bounds(week: u32) -> D {
        let offset = chrono::offset::Local::now();
        let current_year = offset.year();
        let mon = NaiveDate::from_isoywd(current_year, week, Weekday::Mon);
        DateTime::<Utc>::from_utc(mon.and_hms(0, 0, 0), Utc)
    }

    // Convenience method to get the right config file
    fn get_file_path() -> String {
        let dir = dirs::config_dir().unwrap();
        let rabit_dir = format!("{}/{}", dir.to_str().unwrap(), String::from("rabit"));
        create_dir_all(&rabit_dir).unwrap();
        String::from(format!("{}/{}", rabit_dir, FILE_NAME))
    }

    // Store the data on the disk
    pub fn store_state(&self) {
        let file_path = HabitTracker::get_file_path();
        println!("{:?}", file_path);
        let mut file = File::create(file_path).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())
            .unwrap();
    }

    // Fetch all the data from the disk
    pub fn fetch_state() -> Self {
        let file_path = HabitTracker::get_file_path();
        let data = fs::read_to_string(file_path);
        match data {
            Ok(s) => serde_json::from_str(s.as_str()).unwrap(),
            Err(_) => HabitTracker::default(),
        }
    }

    // Default impl
    pub fn default() -> Self {
        HabitTracker {
            start_date: HabitTracker::week_bounds(Utc::now().iso_week().week()),
            habits: vec![],
        }
    }
}
