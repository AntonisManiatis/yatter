use chrono::{Local, Timelike};

/// A time sheet
#[derive(Debug, PartialEq, Eq)]
pub struct TS {
    pub entries: Vec<DateEntry>, // * could have been a Dictionary/Map honestly.
}

impl TS {
    pub fn new() -> Self {
        TS { entries: vec![] }
    }

    pub fn append_entry_for(&mut self, date: String) {
        let date_entry = self.entries.iter_mut().filter(|e| e.date == date).next();

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
                    date,
                    entries: vec![TimeSlot::new(TimeEntry {
                        hour: now.hour(),
                        minute: now.minute(),
                    })],
                };

                self.entries.push(date_entry);
            }
        }
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

/// A time slot is a start [TimeEntry]
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

    pub fn append_entry(&mut self) {
        todo!()
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

#[cfg(test)]
mod tests {
    use super::{DateEntry, TimeEntry, TimeSlot, TS};

    const DATE: &str = "3/18/2023";

    #[test]
    fn appending_a_date_entry_into_an_empty_sheet_also_creates_a_slot_with_a_start_time_for_that_date(
    ) {
        // Arrange
        let mut ts = TS::new();

        // Act
        ts.append_entry_for(DATE.to_string());

        // Assert
        assert_eq!(1, ts.entries.len());
        assert_eq!(1, ts.entries.get(0).unwrap().entries.len())
    }

    #[test]
    fn appending_a_date_entry_to_a_non_empty_sheet_updates_the_slots_end_time_for_that_date() {
        // Arrange
        let slot = TimeSlot::new(TimeEntry {
            hour: 18,
            minute: 16,
        });

        let mut ts = TS {
            entries: vec![DateEntry {
                date: DATE.to_string(),
                entries: vec![slot],
            }],
        };

        // Act
        ts.append_entry_for("3/18/2023".to_string());

        // Assert
        assert_eq!(1, ts.entries.len());
        let expected_date = ts.entries.get(0).unwrap();
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

        let mut ts = TS {
            entries: vec![DateEntry {
                date: DATE.to_string(),
                entries: vec![slot],
            }],
        };

        // Act
        ts.append_entry_for("3/18/2023".to_string());

        // Assert
        assert_eq!(1, ts.entries.len());
        let expected_date = ts.entries.get(0).unwrap();
        assert_eq!(2, expected_date.entries.len());
        let expected_slot = expected_date.entries.get(0).unwrap();
        assert!(expected_slot.end.is_some())
    }
}
