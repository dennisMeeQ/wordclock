use std::error::Error;
use wasm_bindgen::prelude::*;

use chrono::{DateTime, Local};
use pattern::Pattern;

use crate::pattern::time_to_words;

mod led;
mod pattern;
mod time;

pub fn run(print_debug: bool) -> Result<(), Box<dyn Error>> {
    let curr_time: DateTime<Local> = Local::now();
    let curr_time_formatted = format!("{}", curr_time.format("%H:%M:%S"));

    print!("{esc}c", esc = 27 as char);

    let pattern = get_current_pattern();
    println!("{}", pattern.render());

    led::turn_leds_on(&pattern);

    if print_debug {
        println!();
        println!("Words: {}", pattern_to_string(&pattern));

        println!();
        println!("Current time: {}", curr_time_formatted);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn get_current_pattern() -> Pattern {
    let curr_time: DateTime<Local> = Local::now();
    time_to_words(&curr_time).unwrap()
}

#[wasm_bindgen]
pub fn greet() -> String {
    let curr_time: DateTime<Local> = Local::now();
    let pattern = time_to_words(&curr_time);

    match pattern {
        Ok(pat) => pattern_to_string(&pat),
        Err(e) => format!("{:?}", e),
    }
}

fn pattern_to_string(pattern: &Pattern) -> String {
    let mut result = String::from("");

    for line in pattern.fields().as_slice().chunks(pattern.width() as usize) {
        for c in line {
            if c.is_on {
                result.push(c.letter);
            } else {
                if result.len() > 0 && result.chars().last().unwrap() != ' ' {
                    result.push(' ');
                }
            }
        }
    }

    result.trim().to_owned()
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
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST VIER UHR");
    }
    #[test]
    fn converts_time_to_words_5() {
        let time = create_time(16, 04);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST FÜNF NACH VIER");
    }
    #[test]
    fn converts_time_to_words_10() {
        let time = create_time(16, 12);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST ZEHN NACH VIER");
    }
    #[test]
    fn converts_time_to_words_15() {
        let time = create_time(16, 15);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST VIERTEL NACH VIER");
    }
    #[test]
    fn converts_time_to_words_20() {
        let time = create_time(16, 18);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST ZWANZIG NACH VIER");
    }
    #[test]
    fn converts_time_to_words_25() {
        let time = create_time(16, 26);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST FÜNF VOR HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_30() {
        let time = create_time(16, 31);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_35() {
        let time = create_time(16, 37);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST FÜNF NACH HALB FÜNF");
    }
    #[test]
    fn converts_time_to_words_40() {
        let time = create_time(16, 40);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST ZWANZIG VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_45() {
        let time = create_time(16, 44);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST VIERTEL VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_50() {
        let time = create_time(16, 51);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST ZEHN VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_55() {
        let time = create_time(16, 54);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST FÜNF VOR FÜNF");
    }
    #[test]
    fn converts_time_to_words_59() {
        let time = create_time(16, 59);
        let pattern = time_to_words(&time).unwrap();
        let result = pattern_to_string(&pattern);
        assert_eq!(result, "ES IST FÜNF UHR");
    }
}
