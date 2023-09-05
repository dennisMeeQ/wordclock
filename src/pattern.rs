mod de_words;

use chrono::{DateTime, Local, Timelike};
use colored::*;
use std::{error::Error, fmt};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::time::round_time_to_5_min;

use self::de_words::*;

#[wasm_bindgen]
pub struct Field {
    pub letter: char,
    pub is_on: bool,
}

impl Field {
    pub fn new(letter: char) -> Self {
        Field {
            letter,
            is_on: false,
        }
    }
}

// pub type Line = Vec<Field>;
// pub type Pattern = Vec<Line>;

#[wasm_bindgen]
pub struct Pattern {
    width: u32,
    height: u32,
    fields: Vec<Field>,
}

impl Pattern {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[wasm_bindgen]
impl Pattern {
    pub fn new() -> Pattern {
        let width = 11;
        let height = 10;

        let fields = vec![
            // vec![
            Field::new('E'),
            Field::new('S'),
            Field::new('K'),
            Field::new('I'),
            Field::new('S'),
            Field::new('T'),
            Field::new('L'),
            Field::new('F'),
            Field::new('Ü'),
            Field::new('N'),
            Field::new('F'),
            // ],
            // vec![
            Field::new('Z'),
            Field::new('E'),
            Field::new('H'),
            Field::new('N'),
            Field::new('Z'),
            Field::new('W'),
            Field::new('A'),
            Field::new('N'),
            Field::new('Z'),
            Field::new('I'),
            Field::new('G'),
            // ],
            // vec![
            Field::new('D'),
            Field::new('R'),
            Field::new('E'),
            Field::new('I'),
            Field::new('V'),
            Field::new('I'),
            Field::new('E'),
            Field::new('R'),
            Field::new('T'),
            Field::new('E'),
            Field::new('L'),
            // ],
            // vec![
            Field::new('T'),
            Field::new('G'),
            Field::new('N'),
            Field::new('A'),
            Field::new('C'),
            Field::new('H'),
            Field::new('V'),
            Field::new('O'),
            Field::new('R'),
            Field::new('J'),
            Field::new('M'),
            // ],
            // vec![
            Field::new('H'),
            Field::new('A'),
            Field::new('L'),
            Field::new('B'),
            Field::new('Q'),
            Field::new('Z'),
            Field::new('W'),
            Field::new('Ö'),
            Field::new('L'),
            Field::new('F'),
            Field::new('P'),
            // ],
            // vec![
            Field::new('Z'),
            Field::new('W'),
            Field::new('E'),
            Field::new('I'),
            Field::new('N'),
            Field::new('S'),
            Field::new('I'),
            Field::new('E'),
            Field::new('B'),
            Field::new('E'),
            Field::new('N'),
            // ],
            // vec![
            Field::new('K'),
            Field::new('D'),
            Field::new('R'),
            Field::new('E'),
            Field::new('I'),
            Field::new('R'),
            Field::new('H'),
            Field::new('F'),
            Field::new('Ü'),
            Field::new('N'),
            Field::new('F'),
            // ],
            // vec![
            Field::new('E'),
            Field::new('L'),
            Field::new('F'),
            Field::new('N'),
            Field::new('E'),
            Field::new('U'),
            Field::new('N'),
            Field::new('V'),
            Field::new('I'),
            Field::new('E'),
            Field::new('R'),
            // ],
            // vec![
            Field::new('W'),
            Field::new('A'),
            Field::new('C'),
            Field::new('H'),
            Field::new('T'),
            Field::new('Z'),
            Field::new('E'),
            Field::new('H'),
            Field::new('N'),
            Field::new('R'),
            Field::new('S'),
            // ],
            // vec![
            Field::new('B'),
            Field::new('S'),
            Field::new('E'),
            Field::new('C'),
            Field::new('H'),
            Field::new('S'),
            Field::new('F'),
            Field::new('M'),
            Field::new('U'),
            Field::new('H'),
            Field::new('R'),
            // ],
        ];

        Pattern {
            width,
            height,
            fields,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn get_fields(&self) -> *const Field {
        self.fields.as_ptr()
    }

    pub fn get_field_letter(&self, x: usize, y: usize) -> char {
        let field = &self.fields[y * self.width as usize + x];

        field.letter
    }

    pub fn get_field_status(&self, x: usize, y: usize) -> bool {
        let field = &self.fields[y * self.width as usize + x];

        field.is_on
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.fields.as_slice().chunks(self.width as usize) {
            for field in line {
                let symbol = String::from(field.letter);
                if field.is_on {
                    write!(f, "{} ", symbol.bright_green())?;
                } else {
                    write!(f, "{} ", symbol.white().dimmed())?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub struct PatternWord {
    line: usize,
    columns: Vec<usize>,
}

// fn load_base_pattern() -> Result<Pattern, Box<dyn Error>> {
//     let contents = fs::read_to_string("src/pattern/de.txt")?;

//     let mut pattern: Pattern = Pattern::new();

//     for line in contents.lines() {
//         let mut curr_line: Line = vec![];

//         for letter in line.chars() {
//             curr_line.push(Field::new(letter));
//         }

//         pattern.fields.push(curr_line);
//     }

//     // display_pattern(&pattern, true);

//     Ok(pattern)
// }

fn compile_pattern(words: &Vec<PatternWord>) -> Pattern {
    let mut pattern = Pattern::new();

    // // Deep clone
    // for base_line in &base_pattern.fields {
    //     let mut line = vec![];
    //     for base_char in base_line {
    //         line.push(Field::new(base_char.letter))
    //     }
    //     pattern.fields.push(line);
    // }

    // Turn necessary fields on
    for word in words {
        for c in &word.columns {
            pattern.fields[word.line * pattern.width as usize + c].is_on = true;
        }
    }

    pattern
}

pub fn time_to_words(time: &DateTime<Local>) -> Result<Pattern, Box<dyn Error>> {
    // let base_pattern = load_base_pattern()?;
    let words = convert_time_to_words(time);
    let pattern = compile_pattern(&words);

    Ok(pattern)
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

    // #[test]
    // fn loads_the_pattern() {
    //     let result = load_base_pattern().unwrap();
    //     assert_eq!(result.fields[7][7].letter, 'V');
    // }
}
