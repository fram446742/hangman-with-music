use std::collections::HashSet;
use termcolor::{ Color, ColorSpec, StandardStream, WriteColor };

use crate::{
    consts::{ STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7, STAGE_8 },
    lang::{ Language, LanguageData },
    tools::{ clear, print_colored_messages_based_locale_simplified, print_colored_text },
};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Hangman {
    pub history: HashSet<char>,
    pub word: String,
    pub hidden_letter: String,
    pub attempt: Option<char>,
    pub lives: u8,
    pub initial_lives: u8,
    pub color: Color,
    pub stages: Vec<&'static str>,
    // pub language: Language,
    pub language_data: LanguageData,
    pub difficulty: i8,
}

#[allow(dead_code)]
impl Hangman {
    // Constructor to create a new Hangman game
    pub fn new(initial_lives: u8, language_data: LanguageData, color: Color) -> Hangman {
        let word = language_data.get_random_word().unwrap_or(String::from("TestWord")); // Get a random word based on the language
        let hidden_letter = "_ ".repeat(word.len());
        Hangman {
            history: HashSet::new(),
            word,
            hidden_letter,
            attempt: None,
            lives: initial_lives,
            initial_lives,
            color,
            stages: Hangman::initialize_stages(initial_lives),
            // language,
            language_data,
            difficulty: 4,
        }
    }

    // Private method to initialize stages based on initial_lives
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
            _ => vec![STAGE_0], // Default stage if initial_lives is unknown
        }
    }

    // Make a guess and update the game state
    pub fn guess(&mut self, letter: char) -> bool {
        if self.history.contains(&letter) {
            self.display(Some(&self.language_data.get_message("LetterAlreadyUsed").unwrap())); // Use specific message ID
            return false; // Already guessed this letter
        }

        self.history.insert(letter);

        if self.word.contains(letter) {
            self.update_hidden_letter();
            true
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.display(Some(&self.language_data.get_message("IncorrectLetter").unwrap())); // Use specific message ID
            false
        }
    }

    // Check if the game is won
    pub fn is_won(&self) -> bool {
        self.hidden_letter == self.word
    }

    // Check if the game is lost
    pub fn is_lost(&self) -> bool {
        self.lives == 0
    }

    // Reveal the hidden letters based on the latest guess
    fn update_hidden_letter(&mut self) {
        self.hidden_letter = self.word
            .chars()
            .map(|c| if self.history.contains(&c) { c } else { '_' })
            .collect();
    }

    // Refresh the hidden letters based on the latest attempt
    pub fn refresh_line(&mut self) {
        if let Some(letter) = self.attempt {
            self.history.insert(letter);
            self.update_hidden_letter();
        }
    }

    // Change the word and reset the game state
    pub fn change_word(&mut self, new_word: String) {
        self.word = new_word.clone();
        self.hidden_letter = "_ ".repeat(new_word.len());
        self.history.clear();
        self.lives = self.initial_lives;
        self.stages = Hangman::initialize_stages(self.initial_lives);
    }

    // Display the current game state
    // BUG: The hangman draw is not displayed correctly
    pub fn display(&self, message: Option<&str>) {
        let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
        clear();

        let stage = if let Some(stage) = self.stages.get(self.lives as usize) {
            stage
        } else {
            &STAGE_0
        };
        print_colored_text(&mut stdout, stage, self.color);

        // Conditionally print the message if it's Some
        if let Some(msg) = message {
            print_colored_text(&mut stdout, msg, self.color);
        }

        self.print_message_with_extras(
            &mut stdout,
            "WordDisplay",
            &self.hidden_letter,
            None,
            self.color
        );

        self.print_message_with_extras(
            &mut stdout,
            "Lives",
            &self.lives.to_string(),
            None,
            self.color
        );

        self.print_message_with_extras(
            &mut stdout,
            "GuessedLetters",
            &self.history.iter().collect::<String>(),
            None,
            self.color
        );
    }

    // // Ask the user to select a language
    // fn ask_language() -> Language {
    //     // FIXME: Loading Global causes a panic WHYYYYY
    //     let language = LanguageData::load(Language::English); // Load English messages before asking for language
    //     let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    //     print_colored_messages_simplified(&mut stdout, "LanguageMenu", &language, Color::White);
    //     let input = crate::tools::read_input().trim().to_uppercase();

    //     match input.as_str() {
    //         "1" => Language::English,
    //         "2" => Language::Spanish,
    //         "3" => Language::French,
    //         "4" => Language::German,
    //         "5" => Language::Italian,
    //         "6" => Language::Portuguese,
    //         "7" => Language::Russian,
    //         "8" => Language::Chinese,
    //         "9" => Language::Japanese,
    //         _ => Language::English, // Default language if input is unknown
    //     }
    // }

    // // TODO: Implement a method to set the language
    pub fn set_language(&mut self, language: Language) {
        let language_data = LanguageData::load(language);
        self.language_data = language_data;
    }

    // TODO: Implement a method to set the difficulty level
    pub fn set_difficulty(&mut self, difficulty: i8) {
        self.difficulty = difficulty;
    }

    pub fn print_colored(
        &self,
        stdout: &mut StandardStream,
        text: &str,
        color: Color
    ) -> Result<(), std::io::Error> {
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(color));

        stdout.set_color(&color_spec)?;
        writeln!(stdout, "{}", text)?;
        stdout.reset()?;

        Ok(())
    }

    pub fn print_message(
        &self,
        stdout: &mut StandardStream,
        message: &str,
        language_data: Option<&LanguageData>,
        color: Color
    ) {
        let language_data = match language_data {
            Some(data) => data,
            None => &self.language_data,
        };
        let message = language_data
            .get_message(message)
            .unwrap_or_else(|_| format!("Failed to load message with key: {}", message));
        let _ = self.print_colored(stdout, &message, color);
    }

    pub fn print_message_with_extras(
        &self,
        stdout: &mut StandardStream,
        message: &str,
        extras: &str,
        language_data: Option<&LanguageData>,
        color: Color
    ) {
        let language_data = match language_data {
            Some(data) => data,
            None => &self.language_data,
        };
        let message = language_data
            .get_message(message)
            .unwrap_or_else(|_| format!("Failed to load message with key: {}", message));
        let message = format!("{} {}", message, extras);
        let _ = self.print_colored(stdout, &message, color);
    }
}
