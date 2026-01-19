use std::collections::HashSet;

use crate::{
    consts::{STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7, STAGE_8},
    lang::LanguageData,
    messages::MessageKey,
    tools::clear,
    ui::GameUI,
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
        let word = language_data
            .get_random_word()
            .unwrap_or_else(|| String::from("TestWord"))
            .to_ascii_uppercase();
        let hidden_letter = "_".repeat(word.chars().count());
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
            8 => vec![
                STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7, STAGE_8,
            ],
            6 => vec![
                STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6,
            ],
            4 => vec![STAGE_0, STAGE_2, STAGE_4, STAGE_5, STAGE_6],
            2 => vec![STAGE_0, STAGE_2, STAGE_6],
            1 => vec![STAGE_0, STAGE_6],
            _ => vec![STAGE_0],
        }
    }

    pub fn guess(&mut self, letter: char, printer: &mut dyn GameUI) -> bool {
        let letter = letter.to_ascii_uppercase();

        if self.history.contains(&letter) {
            self.display(Some(MessageKey::LetterAlreadyUsed), printer);
            return false;
        }

        self.history.insert(letter);

        if self.word.contains(letter) {
            self.update_hidden_letter();
            self.display(Some(MessageKey::AcceptedLetter), printer);
            true
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.display(Some(MessageKey::IncorrectLetter), printer);
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
        self.hidden_letter = self
            .word
            .chars()
            .map(|c| if self.history.contains(&c) { c } else { '_' })
            .collect::<String>();
    }

    pub fn change_word(&mut self, new_word: String) {
        self.word = new_word.to_ascii_uppercase();
        self.hidden_letter = "_".repeat(self.word.chars().count());
        self.history.clear();
        self.lives = self.initial_lives;
        self.stages = Hangman::initialize_stages(self.initial_lives);
    }

    pub fn display(&mut self, message: Option<MessageKey>, printer: &mut dyn GameUI) {
        clear();
        // Determine stage index based on remaining lives (progress from 0)
        let idx = self.initial_lives.saturating_sub(self.lives) as usize;
        let stage = self.stages.get(idx).unwrap_or(&STAGE_0);
        let _ = printer.print_colored(stage, None, false);

        if let Some(msg) = message {
            let _ = printer.print_message(msg, None, None, false);
        }

        // Create a spaced representation for display, e.g. "_ A _ B"
        let display_hidden = self
            .hidden_letter
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let _ = printer.print_message(MessageKey::WordDisplay, None, Some(&display_hidden), false);
        let _ = printer.print_message(MessageKey::Lives, None, Some(&self.lives.to_string()), false);

        let mut guessed: Vec<char> = self.history.iter().copied().collect();
        guessed.sort();
        let guessed_str = guessed.into_iter().collect::<String>();
        let _ = printer.print_message(MessageKey::GuessedLetters, None, Some(&guessed_str), false);
    }
}
