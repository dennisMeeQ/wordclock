use super::PatternWord;

pub enum Words {
    Es,
    Ist,
    FuenfMinute,
    ZehnMinute,
    ZwanzigMinute,
    Viertel,
    Nach,
    Vor,
    Halb,
    ZweiHour,
    EinsHour,
    DreiHour,
    VierHour,
    FuenfHour,
    SechsHour,
    SiebenHour,
    AchtHour,
    NeunHour,
    ZehnHour,
    ElfHour,
    ZwoelfHour,
    Uhr,
}

pub fn get_word(word: Words) -> PatternWord {
    match word {
        Words::Es => PatternWord {
            line: 0,
            columns: vec![0, 1],
        },
        Words::Ist => PatternWord {
            line: 0,
            columns: vec![3, 4, 5],
        },
        Words::FuenfMinute => PatternWord {
            line: 0,
            columns: vec![7, 8, 9, 10],
        },
        Words::ZehnMinute => PatternWord {
            line: 1,
            columns: vec![0, 1, 2, 3],
        },
        Words::ZwanzigMinute => PatternWord {
            line: 1,
            columns: vec![4, 5, 6, 7, 8, 9, 10],
        },
        Words::Viertel => PatternWord {
            line: 2,
            columns: vec![4, 5, 6, 7, 8, 9, 10],
        },
        Words::Nach => PatternWord {
            line: 3,
            columns: vec![2, 3, 4, 5],
        },
        Words::Vor => PatternWord {
            line: 3,
            columns: vec![6, 7, 8],
        },
        Words::Halb => PatternWord {
            line: 4,
            columns: vec![0, 1, 2, 3],
        },
        Words::ZwoelfHour => PatternWord {
            line: 4,
            columns: vec![5, 6, 7, 8, 9],
        },
        Words::ZweiHour => PatternWord {
            line: 5,
            columns: vec![0, 1, 2, 3],
        },
        Words::EinsHour => PatternWord {
            line: 5,
            columns: vec![2, 3, 4, 5],
        },
        Words::SiebenHour => PatternWord {
            line: 5,
            columns: vec![5, 6, 7, 8, 9, 10],
        },
        Words::DreiHour => PatternWord {
            line: 6,
            columns: vec![1, 2, 3, 4],
        },
        Words::FuenfHour => PatternWord {
            line: 6,
            columns: vec![7, 8, 9, 10],
        },
        Words::ElfHour => PatternWord {
            line: 7,
            columns: vec![0, 1, 2],
        },
        Words::NeunHour => PatternWord {
            line: 7,
            columns: vec![3, 4, 5, 6],
        },
        Words::VierHour => PatternWord {
            line: 7,
            columns: vec![7, 8, 9, 10],
        },
        Words::AchtHour => PatternWord {
            line: 8,
            columns: vec![1, 2, 3, 4],
        },
        Words::ZehnHour => PatternWord {
            line: 8,
            columns: vec![5, 6, 7, 8],
        },
        Words::SechsHour => PatternWord {
            line: 9,
            columns: vec![1, 2, 3, 4, 5],
        },
        Words::Uhr => PatternWord {
            line: 9,
            columns: vec![8, 9, 10],
        },
    }
}
