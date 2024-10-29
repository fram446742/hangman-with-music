use std::collections::HashSet;

use crate::{
    consts::{ STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7, STAGE_8 },
    game::GameData,
    lang::LanguageData,
    printer::ColorfulWriter,
    tools::clear,
};

pub struct Hangman {
    pub history: HashSet<char>,
    pub word: String,
    pub hidden_letter: String,
    pub lives: u8,
    pub initial_lives: u8,
    pub stages: Vec<&'static str>,
}

impl Hangman {
    pub fn new(initial_lives: u8, language_data: LanguageData) -> Hangman {
        let word = language_data.get_random_word().unwrap_or_else(|| String::from("TestWord"));
        let hidden_letter = "_ ".repeat(word.len());
        Hangman {
            history: HashSet::new(),
            word,
            hidden_letter,
            lives: initial_lives,
            initial_lives,
            stages: Self::initialize_stages(initial_lives),
        }
    }

    fn initialize_stages(initial_lives: u8) -> Vec<&'static str> {
        match initial_lives {
            8 =>
                vec![
                    STAGE_0,
                    STAGE_1,
                    STAGE_2,
                    STAGE_3,
                    STAGE_4,
                    STAGE_5,
                    STAGE_6,
                    STAGE_7,
                    STAGE_8
                ],
            6 => vec![STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6],
            4 => vec![STAGE_0, STAGE_2, STAGE_4, STAGE_5, STAGE_6],
            2 => vec![STAGE_0, STAGE_2, STAGE_6],
            1 => vec![STAGE_0, STAGE_6],
            _ => vec![STAGE_0],
        }
    }

    pub fn guess(&mut self, letter: char, printer: &mut ColorfulWriter) -> bool {
        if self.history.contains(&letter) {
            self.display(Some("LetterAlreadyUsed"), printer);
            return false;
        }

        self.history.insert(letter);

        if self.word.contains(letter) {
            self.update_hidden_letter();
            self.display(Some("AcceptedLetter"), printer);
            true
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.display(Some("IncorrectLetter"), printer);
            false
        }
    }

    pub fn is_won(&self) -> bool {
        self.hidden_letter == self.word
    }

    pub fn is_lost(&self) -> bool {
        self.lives == 0
    }

    fn update_hidden_letter(&mut self) {
        self.hidden_letter = self.word
            .chars()
            .map(|c| if self.history.contains(&c) { c } else { '_' })
            .collect::<String>();
    }

    pub fn change_word(&mut self, new_word: String) {
        self.word = new_word.clone();
        self.hidden_letter = "_ ".repeat(new_word.len());
        self.history.clear();
        self.lives = self.initial_lives;
        self.stages = Hangman::initialize_stages(self.initial_lives);
    }

    pub fn display(&mut self, message: Option<&str>, printer: &mut ColorfulWriter) {
        clear();

        let stage = self.stages.get(self.lives as usize).unwrap_or(&STAGE_0);
        let _ = printer.print_colored(stage, None, false);

        if let Some(msg) = message {
            let _ = printer.print_message(msg, None, None::<String>, false);
        } else {
            // let _ = printer.print_message(stage, None, None::<String>, false);
        }

        let _ = printer.print_message("WordDisplay", None, Some(&self.hidden_letter), false);
        let _ = printer.print_message("Lives", None, Some(&self.lives.to_string()), false);
        let _ = printer.print_message(
            "GuessedLetters",
            None,
            Some(&self.history.iter().collect::<String>()),
            false
        );
    }
}
