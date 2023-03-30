use std::{collections::BTreeMap, num::ParseIntError};

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

/// Converts an entity to text.
pub trait ToText {
    fn to_text(&self) -> String;
}

#[derive(Debug)]
pub struct ParsingError(); // TODO: What else does this need?

impl TS {
    pub fn parse(text: &str) -> Result<TS, ParsingError> {
        if text.is_empty() {
            return Ok(TS::new());
        }

        // TODO: Can't I just make collect work for DateEntry?
        let mut entries: BTreeMap<String, DateEntry> = BTreeMap::new();

        // ? Can't I just trim all whitespaces at the start? we'll refactor this later.
        text.lines()
            .filter(|line| {
                line.trim_start_matches(SPACE_CHARACTER) // Is this okay also??
                    .starts_with(LINE_START_CHARACTER)
            })
            .for_each(|line| {
                let date = line.trim_start_matches(LINE_START_CHARACTER).trim(); // is this okay??

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
                                continue;
                            }

                            let slot = TimeSlot::parse(t);

                            t_entries.push(slot.unwrap()); // TODO: No unwraps please
                        }
                    });

                // ? I guess I cannot avoid the 2 string copies huh?
                entries.insert(
                    date.to_owned(),
                    DateEntry {
                        date: date.to_owned(),
                        entries: t_entries,
                    },
                );
            });

        Ok(TS { entries })
    }
}

impl ToText for TS {
    fn to_text(&self) -> String {
        let mut content = String::new();

        for (_date, e) in &self.entries {
            content.push_str(LINE_START_CHARACTER);
            content.push_str(SPACE_CHARACTER);
            content.push_str(&e.date);
            content.push_str(NEW_LINE);

            for te in &e.entries {
                content.push_str(INDENTATION_CHARACTER);
                content.push_str(LINE_START_CHARACTER);
                content.push_str(SPACE_CHARACTER);
                content.push_str(&te.to_text());
                content.push_str(NEW_LINE);
            }
        }

        content
    }
}

impl TimeSlot {
    fn parse(line: &str) -> Result<TimeSlot, ParsingError> {
        let mut slot = line.split(TIME_SLOT_SPLIT_CHARACTER); // TODO: take care if there's or isn't a whitespace.

        let start = slot.next().ok_or(ParsingError())?; // TODO: What error?
        let end = slot.next().ok_or(ParsingError())?; // TODO: What error?

        Ok(TimeSlot {
            start: Some(Punch::In(TimeEntry::parse(start)?)),
            end: match end.is_empty() {
                true => None,
                false => Some(Punch::Out(TimeEntry::parse(end)?)),
            },
        })
    }
}

impl ToText for TimeSlot {
    fn to_text(&self) -> String {
        // TODO: BAD UNWRAPS!
        let mut buffer = String::new();
        buffer.push_str(&self.start.as_ref().unwrap().to_text());
        buffer.push_str(TIME_SLOT_SPLIT_CHARACTER);

        if let Some(te) = &self.end {
            buffer.push_str(&te.to_text());
        }

        buffer
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_err: ParseIntError) -> Self {
        // TODO: Transform the parsing err.
        ParsingError()
    }
}

impl ToText for Punch {
    fn to_text(&self) -> String {
        match self {
            Punch::In(te) => te.to_text(),
            Punch::Out(te) => te.to_text(),
        }
    }
}

impl TimeEntry {
    pub fn parse(text: &str) -> Result<TimeEntry, ParsingError> {
        let mut slots = text.split(TIME_ENTRY_SPLIT_CHARACTER);

        let hour = slots.next().ok_or(ParsingError())?; // TODO: What error?
        let minute = slots.next().ok_or(ParsingError())?; // TODO: What error?

        Ok(TimeEntry::of(hour, minute)?)
    }
}

impl ToText for TimeEntry {
    fn to_text(&self) -> String {
        let mut buffer = String::new();

        // ? perhaps not the greatest implementation but it works :D
        let mut hour = self.hour.to_string();
        if hour.len() != 2 {
            hour.insert_str(0, "0");
        }

        buffer.push_str(&hour);
        buffer.push_str(TIME_ENTRY_SPLIT_CHARACTER);

        let mut minute = self.minute.to_string();
        if minute.len() != 2 {
            minute.insert_str(0, "0");
        }

        buffer.push_str(&minute);

        buffer
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, vec};

    use crate::entities::*;

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
    fn parsing_a_single_date_line_gives_a_time_sheet_with_one_entry() -> Result<(), ParsingError> {
        // Arrange
        let text = "- 03/21/2023";

        // Act
        let ts = TS::parse(text)?;

        // Assert
        assert_eq!(1, ts.entries.len());
        Ok(())
    }

    // * Idea to group whitespace related tests.
    #[test]
    fn parsing_a_date_line_is_not_affected_by_whitespaces() -> Result<(), ParsingError> {
        // Arrange
        let text = "-03/21/2023";

        // Act
        let ts = TS::parse(text)?;

        // Assert
        assert_eq!("03/21/2023", ts.entries.get("03/21/2023").unwrap().date);
        Ok(())
    }

    #[test]
    fn parsing_a_date_line_that_has_no_slots_is_valid() -> Result<(), ParsingError> {
        // Arrange
        let text = "- 03/21/2023\r\n- 03/22/2023\r\n"; // ? I know rust has raw string literals, I have to look how to use em.

        // Act
        let ts = TS::parse(text)?;

        // Assert
        assert_eq!(2, ts.entries.len());
        Ok(())
    }

    // ! Currently this is expected behaviour,
    // ! but another option could be to just throw a ParsingError unless it's a comment.
    #[test]
    fn parsing_a_line_that_does_not_start_with_a_dash_is_ignored() -> Result<(), ParsingError> {
        // Arrange
        let text = "- 03/21/2023\r\n ignored line\r\n - 03/22/2023\r\n"; // ? I know rust has raw string literals, I have to look how to use em.

        // Act
        let ts = TS::parse(text)?;

        // Assert
        assert_eq!(2, ts.entries.len());
        Ok(())
    }

    #[test]
    fn a_line_() {
        let text = "- 03/14/2023\r\n\t- 10:10 to 13:15\r\n"; // ? I know rust has raw string literals, I have to look how to use em.

        let entry = DateEntry {
            date: "03/14/2023".to_string(),
            entries: vec![TimeSlot {
                start: Some(Punch::In(TimeEntry::of("10", "10").unwrap())),
                end: Some(Punch::Out(TimeEntry::of("13", "15").unwrap())),
            }],
        };

        let mut entries = BTreeMap::new();
        entries.insert("03/14/2023".to_owned(), entry);

        let expected = TS { entries };
        let ts = TS::parse(&text).unwrap();

        assert_eq!(expected, ts);
    }

    #[test]
    fn try_parsing_a_valid_time_entry() {
        let time_entry = "10:10";

        let actual = TimeEntry::parse(&time_entry);

        let expected = TimeEntry::of("10", "10").unwrap();

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
            start: Some(Punch::In(TimeEntry::of("10", "00").unwrap())),
            end: Some(Punch::Out(TimeEntry::of("15", "00").unwrap())),
        };

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn try_parsing_an_incomplete_time_slot() -> Result<(), ParsingError> {
        let ts = "10:00 to ";

        let actual = TimeSlot::parse(ts)?;

        let expected = TimeSlot {
            start: Some(Punch::In(TimeEntry::of("10", "00").unwrap())),
            end: None,
        };

        assert_eq!(expected, actual);
        Ok(())
    }

    mod printing {
        use std::collections::BTreeMap;

        use chrono::{DateTime, Local};

        use crate::{
            entities::{DateEntry, Punch, TimeEntry, TimeSlot, TS},
            parser::{ParsingError, ToText, INDENTATION_CHARACTER, NEW_LINE},
        };

        #[test]
        fn to_text_for_a_time_entry_prints_time_with_correct_zeros_if_hour_or_minute_is_single_digit(
        ) -> Result<(), ParsingError> {
            // Arrange
            let entry = TimeEntry::of("9", "13")?;

            // Act
            let text = entry.to_text();

            // Assert
            assert_eq!("09:13", text);
            Ok(())
        }

        #[test]
        fn to_text_writes() {
            // rename this test case, it's horrible :D
            let now: DateTime<Local> = Local::now();

            let date = now.format("%m-%d-%Y").to_string();
            let entry = DateEntry {
                date: date.to_string(),
                entries: vec![TimeSlot {
                    start: Some(Punch::In(TimeEntry::of("10", "10").unwrap())),
                    end: Some(Punch::Out(TimeEntry::of("10", "10").unwrap())),
                }],
            };

            let mut entries = BTreeMap::new();
            entries.insert(date.to_owned(), entry);

            let ts = TS { entries };

            let content = ts.to_text();

            let mut expected = String::new();
            expected.push_str(&format!("- {}", &date));
            expected.push_str(NEW_LINE);
            expected.push_str(INDENTATION_CHARACTER);
            expected.push_str("- 10:10 to 10:10");
            expected.push_str(NEW_LINE);

            assert_eq!(expected, content);
        }
    }
}
