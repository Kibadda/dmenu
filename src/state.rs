use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub enum Dir {
    Up,
    Down,
    Same,
}

pub struct State {
    pub input: String,
    pub programs: Vec<Program>,
    pub filtered_programs: Vec<Program>,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub name: String,
    pub cmd: Vec<String>,
}

impl State {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            programs: vec![],
            filtered_programs: vec![],
            index: 0,
        }
    }

    pub fn load_progams(&mut self) {
        self.programs = vec![
            Program {
                name: String::from("Spotify"),
                cmd: vec![String::from("spotify")],
            },
            Program {
                name: String::from("Discord"),
                cmd: vec![String::from("discord")],
            },
            Program {
                name: String::from("Steam"),
                cmd: vec![String::from("steam-runtime")],
            },
            Program {
                name: String::from("Telegram"),
                cmd: vec![String::from("telegram-desktop")],
            },
            Program {
                name: String::from("Dominion"),
                cmd: vec![
                    String::from("steam"),
                    String::from("steam://rungameid/1131620"),
                ],
            },
        ];

        self.filter();
    }

    fn filter(&mut self) {
        let matcher = SkimMatcherV2::default();

        self.filtered_programs = self
            .programs
            .clone()
            .into_iter()
            .filter(|p| matcher.fuzzy_match(&p.name, &self.input).is_some())
            .collect();

        self.move_index(Dir::Same);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.input.len(), new_char);
        self.filter();
    }

    pub fn delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input = self.input.chars().take(self.input.len() - 1).collect();
            self.filter();
        }
    }

    pub fn move_index(&mut self, dir: Dir) {
        let len = self.filtered_programs.len();

        if len == 0 {
            self.index = 0;
        } else {
            self.index = match dir {
                Dir::Up => match self.index == 0 {
                    true => len - 1,
                    false => self.index - 1,
                },
                Dir::Down => match self.index == len - 1 {
                    true => 0,
                    false => self.index + 1,
                },
                Dir::Same => self.index.clamp(0, len - 1),
            }
        };
    }

    pub fn delete_word(&mut self) {
        let words: Vec<&str> = self.input.split_whitespace().collect();

        self.input = match words.len() {
            0 => String::from(""),
            1 => String::from(""),
            _ => match words.split_last() {
                Some((_, rest)) => rest.join(" "),
                None => String::from(""),
            },
        };

        self.filter();
    }
}
