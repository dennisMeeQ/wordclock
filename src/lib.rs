use std::error::Error;

use chrono::{DateTime, Local, Timelike};
use time::round_time_to_5_min;

use crate::pattern::time_to_words;

mod pattern;
mod time;

pub fn run() -> Result<(), Box<dyn Error>> {
    let curr_time: DateTime<Local> = Local::now();
    let curr_time_formatted = format!("{}", curr_time.format("%H:%M:%S"));

    // let wordclock_words = time_to_words(&curr_time);
    time_to_words(&curr_time)?;

    println!("Current time: {}", curr_time_formatted);
    // println!("{}", wordclock_words);

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn create_time(hour: u32, minute: u32) -> DateTime<Local> {
        Local
            .with_ymd_and_hms(2023, 12, 19, hour, minute, 00)
            .unwrap()
    }

    #[test]
    fn converts_time_to_words_0() {
        let time = create_time(16, 01);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST VIER UHR");
    }
    #[test]
    fn converts_time_to_words_5() {
        let time = create_time(16, 04);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST FÜNF NACH VIER");
    }
    #[test]
    fn converts_time_to_words_10() {
        let time = create_time(16, 12);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST ZEHN NACH VIER");
    }
    #[test]
    fn converts_time_to_words_15() {
        let time = create_time(16, 15);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST VIERTEL NACH VIER");
    }
    #[test]
    fn converts_time_to_words_20() {
        let time = create_time(16, 18);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST ZWANZIG NACH VIER");
    }
    #[test]
    fn converts_time_to_words_25() {
        let time = create_time(16, 26);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST FÜNF VOR HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_30() {
        let time = create_time(16, 31);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_35() {
        let time = create_time(16, 37);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST FÜNF NACH HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_40() {
        let time = create_time(16, 40);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST ZWANZIG VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_45() {
        let time = create_time(16, 44);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST VIERTEL VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_50() {
        let time = create_time(16, 51);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST ZEHN VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_55() {
        let time = create_time(16, 54);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST FÜNF VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_59() {
        let time = create_time(16, 59);
        let result = time_to_words(&time);
        assert_eq!(result, "ES IST FÜNF UHR");
    }
}
