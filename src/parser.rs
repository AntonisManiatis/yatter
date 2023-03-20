use crate::entities::*;

// Text constants

/** Token for spaces */
const SPACE_CHARACTER: &str = " ";
/** Token for comments */
// const COMMENT_CHARACTER: &str = "#";
/** Token indicating the start of a line */
const LINE_START_CHARACTER: &str = "-";
const INDENTATION_CHARACTER: &str = "\t";

#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";

/** Token that indicates a time slot start */
const TIME_SLOT_START_SEQUENCE: &str = "\t- "; // ? I wish there was an easy way to join the 2 constants
/** Token required to split 2 time entries. */
const TIME_SLOT_SPLIT_CHARACTER: &str = " to ";
/** Token required to split a time entry. */
const TIME_ENTRY_SPLIT_CHARACTER: &str = ":";

#[derive(Debug)]
pub struct ParsingError(); // TODO: What else does this need?

impl TS {
    pub fn parse(text: &str) -> Result<TS, ParsingError> {
        if text.is_empty() {
            return Ok(TS::new());
        }

        // TODO: Can't I just make collect work for DateEntry?
        let mut entries: Vec<DateEntry> = vec![];

        text.lines()
            .filter(|line| line.starts_with(LINE_START_CHARACTER))
            .for_each(|line| {
                let date = line.trim_start_matches("- ");

                // TODO: Can't I just make collect work for DateEntry?
                let mut t_entries: Vec<TimeSlot> = vec![];

                text.lines()
                    .skip_while(|cline| cline != &line)
                    .skip(1)
                    .take_while(|line| line.starts_with(TIME_SLOT_START_SEQUENCE))
                    .map(|line| line.split(TIME_SLOT_START_SEQUENCE).collect::<Vec<&str>>()) // TODO: Can we not collect?
                    .for_each(|te| {
                        for t in &te {
                            if t.is_empty() {
                                println!("empty");
                                continue;
                            }

                            let slot = TimeSlot::parse(t);

                            t_entries.push(slot.unwrap());
                        }
                    });

                entries.push(DateEntry {
                    date: date.to_string(),
                    entries: t_entries,
                });
            });

        Ok(TS { entries })
    }

    pub fn to_text(ts: &TS) -> String {
        let mut content = String::new();

        for e in &ts.entries {
            content.push_str(LINE_START_CHARACTER);
            content.push_str(SPACE_CHARACTER);
            content.push_str(&e.date);
            content.push_str(NEW_LINE);

            for te in &e.entries {
                content.push_str(INDENTATION_CHARACTER);
                content.push_str(LINE_START_CHARACTER);
                content.push_str(SPACE_CHARACTER);
                content.push_str(&te.to_string());
                content.push_str(NEW_LINE);
            }
        }

        content.push_str(NEW_LINE);

        content
    }
}

impl TimeSlot {
    fn parse(line: &str) -> Result<TimeSlot, ParsingError> {
        let mut slot = line.split(TIME_SLOT_SPLIT_CHARACTER); // TODO: take care if there's or isn't a whitespace.

        let te_1 = slot.next().ok_or(ParsingError())?; // TODO: What error?
        let te_2 = slot.next().ok_or(ParsingError())?; // TODO: What error?

        Ok(TimeSlot {
            start: Some(TimeEntry::parse(te_1)?),
            end: Some(TimeEntry::parse(te_2)?),
        })
    }
}

impl ToString for TimeSlot {
    fn to_string(&self) -> String {
        // TODO: BAD UNWRAPS!
        let mut buffer = String::new();
        buffer.push_str(&self.start.as_ref().unwrap().to_string());
        buffer.push_str(TIME_SLOT_SPLIT_CHARACTER);

        if let Some(te) = &self.end {
            buffer.push_str(&te.to_string());
        }

        buffer
    }
}

impl TimeEntry {
    pub fn parse(text: &str) -> Result<TimeEntry, ParsingError> {
        let mut slots = text.split(TIME_ENTRY_SPLIT_CHARACTER);

        let hour = slots.next().ok_or(ParsingError())?; // TODO: What error?
        let minute = slots.next().ok_or(ParsingError())?; // TODO: What error?

        let hour = hour.parse::<u32>().unwrap(); // TODO: For now
        let minute = minute.parse::<u32>().unwrap(); // TODO: For now

        Ok(TimeEntry { hour, minute })
    }
}

impl ToString for TimeEntry {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.hour.to_string());
        buffer.push_str(":");
        buffer.push_str(&self.minute.to_string());

        buffer
    }
}

#[cfg(test)]
mod tests {
    use std::{time::SystemTime, vec};

    use chrono::{DateTime, Local};

    use crate::{
        entities::*,
        parser::{INDENTATION_CHARACTER, NEW_LINE},
    };

    use super::ParsingError;

    #[test]
    fn parsing_an_empty_text_gives_an_empty_time_sheet() -> Result<(), ParsingError> {
        // Arrange
        let text = "";

        // Act
        let ts = TS::parse(text)?;

        // Assert
        assert_eq!(0, ts.entries.len());
        Ok(())
    }

    #[test]
    fn a_line_() {
        let text = "- 03/14/2023\r\n\t- 10:10 to 13:15\r\n";

        let entry = DateEntry {
            date: "03/14/2023".to_string(),
            entries: vec![TimeSlot {
                start: Some(TimeEntry {
                    hour: "10".parse::<u32>().unwrap(),
                    minute: "10".parse::<u32>().unwrap(),
                }),
                end: Some(TimeEntry {
                    hour: "13".parse::<u32>().unwrap(),
                    minute: "15".parse::<u32>().unwrap(),
                }),
            }],
        };

        let entries = vec![entry];

        let expected = TS { entries };
        let ts = TS::parse(&text).unwrap();

        assert_eq!(expected, ts);
    }

    #[test]
    fn try_parsing_a_valid_time_entry() {
        let time_entry = "10:10";

        let actual = TimeEntry::parse(&time_entry);

        let expected = TimeEntry {
            hour: "10".parse::<u32>().unwrap(),
            minute: "10".parse::<u32>().unwrap(),
        };

        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn parsing_an_incomplete_time_entry_returns_none() {
        let time_entry = "10";

        let actual = TimeEntry::parse(&time_entry);

        assert!(actual.is_err());
    }

    #[test]
    fn try_parsing_a_valid_time_slot() {
        let time_slot = "10:00 to 15:00";

        let actual = TimeSlot::parse(time_slot);

        let expected = TimeSlot {
            start: Some(TimeEntry {
                hour: "10".parse::<u32>().unwrap(),
                minute: "00".parse::<u32>().unwrap(),
            }),
            end: Some(TimeEntry {
                hour: "15".parse::<u32>().unwrap(),
                minute: "00".parse::<u32>().unwrap(),
            }),
        };

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn to_text_writes() {
        // rename this test case, it's horrible :D
        let today: DateTime<Local> = SystemTime::now().into();

        let date = today.format("%m-%d-%Y").to_string();
        let entry = DateEntry {
            date: date.to_string(),
            entries: vec![TimeSlot {
                start: Some(TimeEntry {
                    hour: "10".parse::<u32>().unwrap(),
                    minute: "10".parse::<u32>().unwrap(),
                }),
                end: Some(TimeEntry {
                    hour: "10".parse::<u32>().unwrap(),
                    minute: "10".parse::<u32>().unwrap(),
                }),
            }],
        };

        let entries = vec![entry];
        let ts = TS { entries };

        let content = TS::to_text(&ts);

        let mut expected = String::new();
        expected.push_str(&format!("- {}", &date));
        expected.push_str(NEW_LINE);
        expected.push_str(INDENTATION_CHARACTER);
        expected.push_str("- 10:10 to 10:10");
        expected.push_str(NEW_LINE);
        expected.push_str(NEW_LINE); // Not sure if this is EOF universally.

        assert_eq!(expected, content);
    }
}
