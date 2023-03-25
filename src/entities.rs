use std::{collections::HashMap, fmt::Display, num::ParseIntError};

use chrono::{Local, Timelike};

#[derive(Debug)]
pub enum Punch {
    In(TimeEntry),
    Out(TimeEntry),
}

/// A time sheet
#[derive(Debug, PartialEq, Eq)]
pub struct TS {
    pub entries: HashMap<String, DateEntry>,
}

impl TS {
    pub fn new() -> Self {
        TS {
            entries: HashMap::new(),
        }
    }

    pub fn append_entry_for(&mut self, date: &str) {
        let date_entry = self.entries.get_mut(date);

        let now = Local::now().time();

        // TODO: I hate this nesting there has to be a better way to do this.
        match date_entry {
            Some(date_entry) => {
                // Get the last slot in the iterator
                match date_entry.entries.iter_mut().last() {
                    // Is there a last item?
                    Some(ts) => {
                        // Does it have an end? add a new slot.
                        if let Some(_) = ts.end.as_mut() {
                            date_entry.entries.push(TimeSlot::new(TimeEntry {
                                hour: now.hour(),
                                minute: now.minute(),
                            }));
                        } else {
                            _ = ts.end.insert(TimeEntry {
                                hour: now.hour(),
                                minute: now.minute(),
                            });
                        }
                    }
                    // if not add one.
                    None => date_entry.entries.push(TimeSlot::new(TimeEntry {
                        hour: now.hour(),
                        minute: now.minute(),
                    })),
                }
            }
            None => {
                let date_entry = DateEntry {
                    date: date.to_owned(), // it's okay lets make a copy here.
                    entries: vec![TimeSlot::new(TimeEntry {
                        hour: now.hour(),
                        minute: now.minute(),
                    })],
                };

                self.entries.insert(date_entry.date.to_owned(), date_entry);
            }
        }
    }

    pub fn get_last_punch_for(&mut self, date: &str) -> Option<Punch> {
        let date_entry = self.entries.get(date);

        let now = Local::now().time();

        // TODO: Find this
        Some(Punch::In(TimeEntry { hour: 1, minute: 1 }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    // TODO: Impl a date concept in our domain.
}

#[derive(Debug, PartialEq, Eq)]
pub struct DateEntry {
    pub date: String, // TODO: A strongly typed date?
    pub entries: Vec<TimeSlot>,
}

/// A time slot is a combination of a start [TimeEntry] and an end [TimeEntry]
#[derive(Debug, PartialEq, Eq)]
pub struct TimeSlot {
    pub start: Option<TimeEntry>, // TODO: We can drop the optional here. A time slot will exist when start exists.
    pub end: Option<TimeEntry>,
}

impl TimeSlot {
    pub fn new(start: TimeEntry) -> Self {
        TimeSlot {
            start: Some(start),
            end: None,
        }
    }
}

/// 0 to 23
pub type Hour = u32; // I wish this wast more strict.
/// 0 to 59
pub type Minute = u32; // I wish this wast more strict.

#[derive(Debug, PartialEq, Eq)]
pub struct TimeEntry {
    pub hour: Hour,
    pub minute: Minute,
}

impl TimeEntry {
    pub fn of(hour: &str, minute: &str) -> Result<TimeEntry, ParseIntError> {
        Ok(TimeEntry {
            hour: hour.parse::<u32>()?,
            minute: minute.parse::<u32>()?,
        })
    }
}

impl Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Parser has similar logic.
        write!(f, "{}:{}", self.hour, self.minute)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{DateEntry, TimeEntry, TimeSlot, TS};

    const DATE: &str = "3/18/2023";

    #[test]
    fn appending_a_date_entry_into_an_empty_sheet_also_creates_a_slot_with_a_start_time_for_that_date(
    ) {
        // Arrange
        let mut ts = TS::new();

        // Act
        ts.append_entry_for(DATE);

        // Assert
        assert_eq!(1, ts.entries.len());
        assert_eq!(1, ts.entries.get(DATE).unwrap().entries.len())
    }

    #[test]
    fn appending_a_date_entry_to_a_non_empty_sheet_updates_the_slots_end_time_for_that_date() {
        // Arrange
        let slot = TimeSlot::new(TimeEntry {
            hour: 18,
            minute: 16,
        });

        let mut entries = HashMap::new();
        entries.insert(
            DATE.to_string(),
            DateEntry {
                date: DATE.to_string(),
                entries: vec![slot],
            },
        );

        let mut ts = TS { entries };

        // Act
        ts.append_entry_for("3/18/2023");

        // Assert
        assert_eq!(1, ts.entries.len());
        let expected_date = ts.entries.get(DATE).unwrap();
        assert_eq!(1, expected_date.entries.len());
        let expected_slot = expected_date.entries.get(0).unwrap();
        assert!(expected_slot.end.is_some())
    }

    #[test]
    fn appending_a_date_entry_to_a_non_empty_sheet_that_has_a_slot_completed_creates_another() {
        // Arrange
        let slot = TimeSlot {
            start: Some(TimeEntry {
                hour: 18,
                minute: 16,
            }),
            end: Some(TimeEntry {
                hour: 18,
                minute: 16,
            }),
        };

        let mut entries = HashMap::new();
        entries.insert(
            DATE.to_owned(),
            DateEntry {
                date: DATE.to_string(),
                entries: vec![slot],
            },
        );

        let mut ts = TS { entries };

        // Act
        ts.append_entry_for("3/18/2023");

        // Assert
        assert_eq!(1, ts.entries.len());
        let expected_date = ts.entries.get(DATE).unwrap();
        assert_eq!(2, expected_date.entries.len());
        let expected_slot = expected_date.entries.get(0).unwrap();
        assert!(expected_slot.end.is_some())
    }
}
