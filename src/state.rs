use crate::program::{load_from_dir, Program};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::widgets::ListState;

pub enum Dir {
    Up,
    Down,
    Same,
}

pub struct State {
    pub input: String,
    pub programs: Vec<Program>,
    pub filtered_programs: Vec<Program>,
    pub list_state: ListState,
}

impl State {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            programs: vec![],
            filtered_programs: vec![],
            list_state: ListState::default(),
        }
    }

    pub fn load_progams(&mut self) {
        let mut programs: Vec<Program> = Vec::new();

        [
            // "/home/michael/.local/share/applications",
            // "/usr/share/applications",
            // "/usr/local/share/applications",
            "/run/current-system/sw/share/applications",
            "/etc/profiles/per-user/michael/share/applications",
        ]
        .iter()
        .for_each(|dir| programs.append(&mut load_from_dir(dir)));

        self.programs = programs;

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
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(match dir {
                Dir::Down => Some(match self.list_state.selected() {
                    Some(i) => {
                        if i >= self.filtered_programs.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                }),
                Dir::Up => Some(match self.list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_programs.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                }),
                Dir::Same => Some(match self.list_state.selected() {
                    Some(i) => i.clamp(0, self.filtered_programs.len() - 1),
                    None => 0,
                }),
            });
        };
    }

    pub fn delete_word(&mut self) {
        let words: Vec<&str> = self.input.split_whitespace().collect();

        self.input = match words.len() {
            0 | 1 => String::from(""),
            _ => match words.split_last() {
                Some((_, rest)) => rest.join(" "),
                None => String::from(""),
            },
        };

        self.filter();
    }
}
