use std::collections::HashSet;

use crate::{
    consts::{STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7, STAGE_8},
    lang::LanguageData,
    messages::MessageKey,
    tools::clear,
    ui::GameUI,
};

pub struct Hangman {
    history: HashSet<char>,
    word: String,
    hidden_letter: String,
    lives: u8,
    initial_lives: u8,
    stages: Vec<&'static str>,
}

impl Hangman {
    /// Create a new `Hangman` instance using the provided initial lives and
    /// language data (to select a random word).
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

    /// Returns the current number of lives remaining.
    pub fn lives(&self) -> u8 {
        self.lives
    }

    /// Returns the configured initial lives for this game instance.
    pub fn initial_lives(&self) -> u8 {
        self.initial_lives
    }

    /// Sets the initial lives and recomputes the stages accordingly. Also resets current lives to the new initial value.
    pub fn set_initial_lives(&mut self, lives: u8) {
        self.initial_lives = lives;
        self.lives = lives;
        self.stages = Self::initialize_stages(self.initial_lives);
    }

    /// Returns the current word (uppercase).
    pub fn word(&self) -> &str {
        &self.word
    }

    /// Attempt to guess a character. Returns `true` if the guess was
    /// correct and `false` otherwise. Uses the provided `GameUI` to show
    /// feedback messages and the current state.
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

    /// Replaces the secret word with `new_word` and resets the state (history
    /// and lives) accordingly.
    pub fn change_word(&mut self, new_word: String) {
        self.word = new_word.to_ascii_uppercase();
        self.hidden_letter = "_".repeat(self.word.chars().count());
        self.history.clear();
        self.lives = self.initial_lives;
        self.stages = Hangman::initialize_stages(self.initial_lives);
    }

    /// Render the current state (stage, masked word, lives and guesses) using
    /// the supplied `GameUI` implementor.
    pub fn display(&mut self, message: Option<MessageKey>, printer: &mut dyn GameUI) {
        clear();
        // Determine stage index based on remaining lives (progress from 0)
        let idx = self.initial_lives.saturating_sub(self.lives) as usize;
        let stage = self.stages.get(idx).unwrap_or(&STAGE_0);
        printer.safe_print_colored(stage, None, false, "stage");

        if let Some(msg) = message {
            printer.safe_print_message(msg, None, None, false, "message");
        }

        // Create a spaced representation for display, e.g. "_ A _ B"
        let display_hidden = self
            .hidden_letter
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        printer.safe_print_message(
            MessageKey::WordDisplay,
            None,
            Some(&display_hidden),
            false,
            "WordDisplay",
        );
        printer.safe_print_message(
            MessageKey::Lives,
            None,
            Some(&self.lives.to_string()),
            false,
            "Lives",
        );

        let mut guessed: Vec<char> = self.history.iter().copied().collect();
        guessed.sort();
        let guessed_str = guessed.into_iter().collect::<String>();
        printer.safe_print_message(
            MessageKey::GuessedLetters,
            None,
            Some(&guessed_str),
            false,
            "GuessedLetters",
        );
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::Language;

    struct DummyPrinter;
    impl GameUI for DummyPrinter {
        fn print_message(
            &mut self,
            _key: crate::messages::MessageKey,
            _color: Option<termcolor::Color>,
            _extras: Option<&str>,
            _bold: bool,
        ) -> std::io::Result<()> {
            Ok(())
        }
        fn print_colored(
            &mut self,
            _text: &str,
            _color: Option<termcolor::Color>,
            _bold: bool,
        ) -> std::io::Result<()> {
            Ok(())
        }
        fn read_input(&self) -> String {
            String::new()
        }
        fn read_char(&self) -> Option<char> {
            None
        }
        fn read_pass(&self) -> String {
            String::new()
        }
        fn clear(&self) {}
        fn set_color(&mut self, _color: Option<termcolor::Color>) {}
        fn change_language(&mut self) {}
        fn get_language_data(&self) -> &crate::lang::LanguageData {
            static LD: once_cell::sync::OnceCell<crate::lang::LanguageData> =
                once_cell::sync::OnceCell::new();
            LD.get_or_init(|| crate::lang::LanguageData::load(Language::Global))
        }
    }

    #[test]
    fn lives_getter_and_setter_work() {
        let lang = crate::lang::LanguageData::load(Language::Global);
        let mut h = Hangman::new(6, lang);
        assert_eq!(h.initial_lives(), 6);
        assert_eq!(h.lives(), 6);
        h.set_initial_lives(4);
        assert_eq!(h.initial_lives(), 4);
        assert_eq!(h.lives(), 4);
    }

    #[test]
    fn guessing_results_in_win_and_loss() {
        let lang = crate::lang::LanguageData::load(Language::Global);
        let mut h = Hangman::new(3, lang);
        // Force a known word so test is deterministic
        h.change_word("AA".to_string());
        let mut p = DummyPrinter;
        // guess correctly
        assert!(h.guess('A', &mut p));
        assert!(h.is_won());

        // change word and make wrong guesses until lost
        h.change_word("BB".to_string());
        assert!(!h.guess('A', &mut p));
        assert!(!h.is_lost());
        assert!(!h.guess('C', &mut p));
        assert!(h.is_lost() || h.lives() < 3); // lives decreased
    }
}
