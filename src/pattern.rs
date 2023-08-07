mod de_words;

use chrono::{DateTime, Local, Timelike};
use colored::*;
use std::{error::Error, fs};

use crate::time::round_time_to_5_min;

use self::de_words::*;

pub struct Field {
    letter: char,
    is_on: bool,
}

impl Field {
    pub fn new(letter: char) -> Self {
        Field {
            letter,
            is_on: false,
        }
    }
}

pub type Line = Vec<Field>;
pub type Pattern = Vec<Line>;

pub struct PatternWord {
    line: usize,
    columns: Vec<usize>,
}

fn display_pattern(pattern: &Pattern, show_all_chars: bool) {
    println!();
    println!();
    println!();

    for line in pattern {
        for field in line {
            if field.is_on || show_all_chars {
                print!("{} ", String::from(field.letter).bright_green())
            } else {
                print!("{} ", String::from(field.letter).white().dimmed())
            }
        }
        println!();
    }

    println!();
    println!();
    println!();
}

fn load_base_pattern() -> Result<Pattern, Box<dyn Error>> {
    let contents = fs::read_to_string("src/pattern/de.txt")?;

    let mut pattern: Pattern = vec![];

    for line in contents.lines() {
        let mut curr_line: Line = vec![];

        for letter in line.chars() {
            curr_line.push(Field::new(letter));
        }

        pattern.push(curr_line);
    }

    // display_pattern(&pattern, true);

    Ok(pattern)
}

fn compile_pattern(base_pattern: &Pattern, words: &Vec<PatternWord>) -> Pattern {
    let mut pattern = vec![];

    // Deep clone
    for base_line in base_pattern {
        let mut line = vec![];
        for base_char in base_line {
            line.push(Field::new(base_char.letter))
        }
        pattern.push(line);
    }

    // Turn necessary fields on
    for word in words {
        for c in &word.columns {
            pattern[word.line][*c].is_on = true;
        }
    }

    pattern
}

pub fn time_to_words(time: &DateTime<Local>) -> Result<(), Box<dyn Error>> {
    let base_pattern = load_base_pattern()?;
    let words = convert_time_to_words(time);
    let pattern = compile_pattern(&base_pattern, &words);

    display_pattern(&pattern, false);

    Ok(())
}

fn convert_time_to_words(time: &DateTime<Local>) -> Vec<PatternWord> {
    let rounded_time = round_time_to_5_min(&time);

    let (_, hour) = rounded_time.hour12();
    let minute = rounded_time.minute();

    let mut words: Vec<PatternWord> = vec![];

    // Prefix
    words.push(get_word(Words::Es));
    words.push(get_word(Words::Ist));

    // Minutes
    match minute {
        5 => {
            words.push(get_word(Words::FuenfMinute));
            words.push(get_word(Words::Nach));
        }
        10 => {
            words.push(get_word(Words::ZehnMinute));
            words.push(get_word(Words::Nach));
        }
        15 => {
            words.push(get_word(Words::Viertel));
            words.push(get_word(Words::Nach));
        }
        20 => {
            words.push(get_word(Words::ZwanzigMinute));
            words.push(get_word(Words::Nach));
        }
        25 => {
            words.push(get_word(Words::FuenfMinute));
            words.push(get_word(Words::Vor));
            words.push(get_word(Words::Halb));
        }
        30 => {
            words.push(get_word(Words::Halb));
        }
        35 => {
            words.push(get_word(Words::FuenfMinute));
            words.push(get_word(Words::Nach));
            words.push(get_word(Words::Halb));
        }
        40 => {
            words.push(get_word(Words::ZwanzigMinute));
            words.push(get_word(Words::Vor));
        }
        45 => {
            words.push(get_word(Words::Viertel));
            words.push(get_word(Words::Vor));
        }
        50 => {
            words.push(get_word(Words::ZehnMinute));
            words.push(get_word(Words::Vor));
        }
        55 => {
            words.push(get_word(Words::FuenfMinute));
            words.push(get_word(Words::Vor));
        }
        _ => (),
    };

    // HOUR
    let mut hour_for_word = hour;
    if minute > 20 {
        hour_for_word = hour_for_word + 1;
    }

    match hour_for_word {
        1 => words.push(get_word(Words::EinsHour)),
        2 => words.push(get_word(Words::ZweiHour)),
        3 => words.push(get_word(Words::DreiHour)),
        4 => words.push(get_word(Words::VierHour)),
        5 => words.push(get_word(Words::FuenfHour)),
        6 => words.push(get_word(Words::SechsHour)),
        7 => words.push(get_word(Words::SiebenHour)),
        8 => words.push(get_word(Words::AchtHour)),
        9 => words.push(get_word(Words::NeunHour)),
        10 => words.push(get_word(Words::ZehnHour)),
        11 => words.push(get_word(Words::ElfHour)),
        12 => words.push(get_word(Words::ZwoelfHour)),
        _ => (),
    };

    // SUFFIX
    if minute == 0 {
        words.push(get_word(Words::Uhr));
    };

    words
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
        let result = convert_time_to_words(&time);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn loads_the_pattern() {
        let result = load_base_pattern().unwrap();
        assert_eq!(result[7][7].letter, 'V');
    }
}
