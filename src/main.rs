mod cli;

use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
    vec,
};

use chrono::{DateTime, Datelike, Local};

use crate::cli::Action;

const ROOT_ARG_POSITION: usize = 1;
const EXPECTED_EXTENSIONS: [&str; 2] = ["", ".txt"];

// Text constants

/** Token for spaces */
const SPACE_CHARACTER: &str = " ";
/** Token for comments */
const COMMENT_CHARACTER: &str = "#";
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

fn main() {
    // TODO: Refactor into function that reads proper file.
    let today: DateTime<Local> = SystemTime::now().into();
    let current_year = today.year();

    // ? Having multiple extensions could cause ambiguities though.
    // ? Can we specify a config val to select which?
    let file_set =
        EXPECTED_EXTENSIONS.map(|ext| format!("{}{}", today.format("%m-%Y").to_string(), ext));

    // We expect a target dir.
    let root_path = env::args()
        .nth(ROOT_ARG_POSITION)
        .expect("Expected a root path.");

    let year_dirs = fs::read_dir(&root_path).expect("root dir doesn't exist");

    let year_path = year_dirs
        .map(|entry| entry.unwrap().path()) // TODO: Unwrap it for now.
        .filter(|path| path.is_dir())
        .filter(|path| path.ends_with(&current_year.to_string())) // TODO: Not ideal, but works :)
        .next();

    // TODO: Current year dir should be created if it doesn't exist.
    if let None = year_path {
        let y_s = current_year.to_string();
        let mut new_path = PathBuf::new();
        new_path.push(&root_path);
        new_path.push(&y_s);
        _ = fs::create_dir(new_path).expect("Failed to create the current year dir.");
    }

    println!("{:?}", year_path);
    let year_paths = fs::read_dir(year_path.unwrap()).unwrap();

    // TODO: Find a file matching the formats we provided.
    // * EG: "03-2023.txt"
    let month_path = year_paths
        .map(|entry| entry.unwrap().path())
        .filter(|pb| pb.ends_with(&PathBuf::from(&file_set[1]))) // TODO: Not ideal, but works :)
        .next();

    println!("{:?}", &month_path);

    if let None = month_path {
        // TODO: Pass in the correct name.
        File::create("./data/2023/1").expect("Failed to create file."); // TODO: Better error
    }

    let pb = month_path.unwrap();

    if let Ok(lines) = fs::read_to_string(&pb) {
        match TS::parse(&lines) {
            Ok(ts) => {
                // TODO: Execute the required logic, append a new entry
                // ts.punch_in();

                // TODO: Write only if there was a change. :)
                // Write the updated TS back to the file
                let content = TS::to_text(&ts);

                _ = fs::write(pb.into_os_string(), content);
            }
            Err(e) => eprint!("{:?}", e),
        };
    };
}

#[derive(Debug, PartialEq, Eq)]
struct TS<'a> {
    entries: Vec<DateEntry<'a>>,
}

#[derive(Debug)]
struct ParsingError(); // TODO: What else does this need?

impl TS<'_> {
    fn parse(text: &str) -> Result<TS, ParsingError> {
        // TODO: Can't I just make collect work for DateEntry?
        let mut entries: Vec<DateEntry<'_>> = vec![];

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
                    date,
                    entries: t_entries,
                });
            });

        Ok(TS { entries })
    }

    fn to_text(ts: &TS) -> String {
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

    // TODO: Option or Result here?
    fn punch_in(&self) {
        todo!();
    }

    // TODO: Option or Result here?
    fn punch_out(&self) {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Date {
    // TODO: Impl a date concept in our domain.
}

#[derive(Debug, PartialEq, Eq)]
struct DateEntry<'a> {
    date: &'a str, // TODO: A strongly typed date?
    entries: Vec<TimeSlot<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
struct TimeSlot<'a> {
    start: TimeEntry<'a>,
    end: TimeEntry<'a>,
}

impl TimeSlot<'_> {
    fn parse(line: &str) -> Result<TimeSlot, ParsingError> {
        let mut slot = line.split(TIME_SLOT_SPLIT_CHARACTER); // TODO: take care if there's or isn't a whitespace.

        let te_1 = slot.next().ok_or(ParsingError())?; // TODO: What error?
        let te_2 = slot.next().ok_or(ParsingError())?; // TODO: What error?

        Ok(TimeSlot {
            start: TimeEntry::parse(te_1)?,
            end: TimeEntry::parse(te_2)?,
        })
    }
}

impl ToString for TimeSlot<'_> {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.start.to_string());
        buffer.push_str(TIME_SLOT_SPLIT_CHARACTER);
        buffer.push_str(&self.end.to_string());

        buffer
    }
}

#[derive(Debug)]
struct Hour(u8);

#[derive(Debug)]
struct Minute(u8);

#[derive(Debug, PartialEq, Eq)]
struct TimeEntry<'a> {
    hour: &'a str,   // ? Replace with Hour?
    minute: &'a str, // ? Replace with Minute?
}

impl TimeEntry<'_> {
    fn parse(text: &str) -> Result<TimeEntry, ParsingError> {
        let mut slots = text.split(TIME_ENTRY_SPLIT_CHARACTER);

        let hour = slots.next().ok_or(ParsingError())?; // TODO: What error?
        let minute = slots.next().ok_or(ParsingError())?; // TODO: What error?

        Ok(TimeEntry { hour, minute })
    }
}

impl ToString for TimeEntry<'_> {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.hour);
        buffer.push_str(":");
        buffer.push_str(&self.minute);

        buffer
    }
}

#[cfg(test)]
mod tests {
    use std::{time::SystemTime, vec};

    use chrono::{DateTime, Local};

    use crate::{DateEntry, TimeEntry, TimeSlot, INDENTATION_CHARACTER, NEW_LINE, TS};

    #[test]
    fn a_line_() {
        let text = "- 03/14/2023\r\n\t- 10:10 to 13:15\r\n";

        let entry = DateEntry {
            date: "03/14/2023",
            entries: vec![TimeSlot {
                start: TimeEntry {
                    hour: "10",
                    minute: "10",
                },
                end: TimeEntry {
                    hour: "13",
                    minute: "15",
                },
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
            hour: "10",
            minute: "10",
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
            start: TimeEntry {
                hour: "10",
                minute: "00",
            },
            end: TimeEntry {
                hour: "15",
                minute: "00",
            },
        };

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn to_text_writes() {
        // rename this test case, it's horrible :D
        let today: DateTime<Local> = SystemTime::now().into();

        let date = today.format("%m-%d-%Y").to_string();
        let entry = DateEntry {
            date: &date,
            entries: vec![TimeSlot {
                start: TimeEntry {
                    hour: "10",
                    minute: "10",
                },
                end: TimeEntry {
                    hour: "10",
                    minute: "10",
                },
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
