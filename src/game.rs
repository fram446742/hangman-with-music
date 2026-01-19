//! Core game logic and interactive flows for Hangman.
//!
//! This module contains `GameData` which owns the game state and orchestrates
//! interactions between the UI, the hangman logic and the music player.

use crate::{
    consts::{EASTEREGG, EASTEREGG2},
    hangman::Hangman,
    lang::LanguageData,
    messages::MessageKey,
    player::MusicPlayer,
    tools::random_color,
    ui::GameUI,
};
use anyhow::Context;
use anyhow::Result;
use rand::Rng;
use std::process::exit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Game {
    Hangman,
    Hangman2Players,
}

enum DifficultyLevel {
    Easy = 8,
    Medium = 6,
    Hard = 3,
    Insane = 1,
}

/// Container for game state and orchestration.
///
/// `GameData` owns the `Hangman` instance(s), the music player and the UI
/// implementation (via `GameUI`). It provides high-level methods to run the
/// game and configure options.
pub struct GameData {
    game: Game,
    hangman: Box<Hangman>,
    hangman2: Option<Box<Hangman>>,
    music_player: MusicPlayer,
    printer: Box<dyn GameUI>,
}

impl GameData {
    /// Crea una nueva instancia preguntando la dificultad y preparando música y palabra.
    pub fn new(printer: Box<dyn GameUI>, two_players: bool) -> Result<Self> {
        Self::new_with_music_dir(printer, two_players, "./music")
    }

    /// Create a new GameData instance but allow specifying the music directory (useful for tests)
    pub fn new_with_music_dir(
        mut printer: Box<dyn GameUI>,
        two_players: bool,
        music_dir: &str,
    ) -> Result<Self> {
        // load language and select difficulty
        let language_data: LanguageData = printer.get_language_data().clone();
        let difficulty = Self::select_difficulty(&mut *printer);

        // create one or two hangman instances depending on mode
        let hangman = Box::new(Hangman::new(difficulty, language_data.clone()));
        let hangman2 = if two_players {
            Some(Box::new(Hangman::new(difficulty, language_data.clone())))
        } else {
            None
        };

        // initialize music using the provided directory
        let mut music = MusicPlayer::new();
        music
            .init_in(music_dir, Some(&mut *printer))
            .context("initializing music player failed")?;
        music.start_music();

        let mut instance = Self {
            game: if two_players {
                Game::Hangman2Players
            } else {
                Game::Hangman
            },
            hangman,
            hangman2,
            music_player: music,
            printer,
        };

        instance.set_difficulty(difficulty);
        Ok(instance)
    }

    fn finished_game(&mut self, won: bool) {
        let word = self.hangman.word().to_string();
        if won {
            self.printer.safe_print(
                Some(MessageKey::Congratulations),
                None,
                None,
                Some(&word),
                false,
                true,
                "Congratulations",
            );
        } else {
            self.printer.safe_print(
                Some(MessageKey::GameOver),
                None,
                None,
                Some(&word),
                false,
                true,
                "GameOver",
            );
        }

        // Continue message sits under the result message and waits for input
        self.printer.safe_print(
            Some(MessageKey::ContinueMessage),
            None,
            None,
            None,
            false,
            false,
            "ContinueMessage",
        );
        let _ = self.printer.read_input();
    }

    /// Runs a single game session. Returns Ok(()) on normal completion or an error if something fails.
    pub fn run(&mut self) -> anyhow::Result<()> {
        match self.game {
            Game::Hangman => {
                self.main_menu();
                self.printer.clear();

                self.hangman.display(None, &mut *self.printer);

                loop {
                    if let Some(c) = self.printer.read_char() {
                        if !c.is_alphabetic() {
                            self.printer.safe_print(
                                Some(MessageKey::InvalidCharacter),
                                None,
                                None,
                                None,
                                false,
                                false,
                                "InvalidCharacter",
                            );
                            continue;
                        }

                        let guess = c.to_ascii_uppercase();
                        let _ = self.hangman.guess(guess, &mut *self.printer);
                    } else {
                        // no input, skip iteration
                        continue;
                    }

                    if self.hangman.is_lost() {
                        self.finished_game(false);
                        break;
                    } else if self.hangman.is_won() {
                        self.finished_game(true);
                        break;
                    }
                }
            }
            Game::Hangman2Players => {
                // two-player mode not implemented yet
                self.printer.safe_print(
                    Some(MessageKey::InvalidOption),
                    None,
                    None,
                    None,
                    false,
                    false,
                    "InvalidOption",
                );
            }
        }

        Ok(())
    }

    /// Prompt the user to retry the game and stop the current music playback.
    ///
    /// Returns `true` if the user answered yes. Regardless of the answer the
    /// current music is cleared so a restarted game won't overlap with the
    /// previous player's playback.
    pub fn ask_retry(&mut self) -> bool {
        self.printer.clear();
        self.printer.safe_print(
            Some(MessageKey::RetryPrompt),
            None,
            None,
            None,
            false,
            false,
            "RetryPrompt",
        );

        let input = self.printer.read_input().trim().to_uppercase();
        let retry = matches!(
            input.as_str(),
            "Y" | "YES" | "S" | "SI" | "J" | "JA" | "O" | "OK"
        );

        // Ensure any currently playing music is stopped and the queue cleared
        // to avoid overlapping playback when a new GameData instance starts.
        self.music_player.clear();

        retry
    }

    pub fn main_menu(&mut self) {
        loop {
            self.printer.safe_print(
                Some(MessageKey::WelcomeBanner),
                None,
                None,
                None,
                false,
                true,
                "WelcomeBanner",
            );
            self.printer.safe_print(
                Some(MessageKey::StartMessage),
                None,
                None,
                None,
                false,
                false,
                "StartMessage",
            );

            let input = self.printer.read_input().trim().to_uppercase();
            if self.handle_main_choice(input.as_str()) {
                continue;
            }

            break;
        }
    }

    fn handle_main_choice(&mut self, choice: &str) -> bool {
        match choice {
            "I" => {
                self.printer.safe_print(
                    Some(MessageKey::Instructions),
                    None,
                    None,
                    None,
                    false,
                    true,
                    "Instructions",
                );
                self.printer.safe_print(
                    Some(MessageKey::ContinueMessage),
                    None,
                    None,
                    None,
                    false,
                    false,
                    "ContinueMessage",
                );
                let _ = self.printer.read_input();
                true
            }
            "S" | "A" => {
                self.printer.clear();
                self.config();
                true
            }
            "E" => {
                exit(0);
            }
            _ => false,
        }
    }

    fn config(&mut self) {
        loop {
            self.printer.safe_print(
                Some(MessageKey::SettingsMenu),
                None,
                None,
                None,
                false,
                true,
                "SettingsMenu",
            );
            let input = self.printer.read_input().trim().to_uppercase();

            if self.handle_config_choice(input.as_str()) {
                continue;
            }

            break;
        }
    }

    fn handle_config_choice(&mut self, choice: &str) -> bool {
        match choice {
            "1" => {
                self.printer.set_color(Some(random_color()));
                true
            }
            "2" => {
                self.music_player.toggle_music();
                true
            }
            "3" => {
                self.set_players();
                true
            }
            "4" => {
                self.printer.change_language();
                let new_word = self
                    .printer
                    .get_language_data()
                    .get_random_word()
                    .unwrap_or_else(|| String::from("TestWord"));
                self.hangman.change_word(new_word);
                true
            }
            "5" => {
                self.set_difficulty_interactive();
                true
            }
            "EASTEREGG" => {
                self.hid();
                true
            }
            _ => false,
        }
    }

    pub fn hid(&mut self) {
        self.printer.safe_print(
            Some(MessageKey::InsertPassword),
            None,
            None,
            None,
            false,
            false,
            "InsertPassword",
        );
        let pass = self.printer.read_pass();
        let pass_ok = pass.eq_ignore_ascii_case("HIDDEN") || pass.eq_ignore_ascii_case("OCULTO");

        if pass_ok {
            self.printer.safe_print(
                Some(MessageKey::AccessGranted),
                None,
                None,
                None,
                false,
                false,
                "AccessGranted",
            );
            self.printer.safe_print(
                Some(MessageKey::EasterEgg1),
                None,
                None,
                None,
                false,
                false,
                "EasterEgg1",
            );
            self.printer.safe_print(
                None,
                Some(EASTEREGG),
                None,
                None,
                false,
                true,
                "EasterEgg",
            );
            self.printer.safe_print(
                Some(MessageKey::ContinueMessage),
                None,
                None,
                None,
                false,
                false,
                "ContinueMessage",
            );
            let _ = self.printer.read_input();

            let mut rng = rand::rng();
            if rng.random_range(0..=5) == 5 {
                self.printer.safe_print(None, Some(EASTEREGG2), None, None, false, false, "EasterEgg2");
                self.printer.safe_print(Some(MessageKey::EasterEgg2), None, None, None, false, false, "EasterEgg2");
                let _ = self.printer.read_input();
            }
        } else {
            self.printer.safe_print(Some(MessageKey::AccessDenied), None, None, None, false, false, "AccessDenied");
            self.printer.safe_print(Some(MessageKey::ContinueMessage), None, None, None, false, false, "ContinueMessage");
            let _ = self.printer.read_input();
        }
    }

    // Extracted helper used by tests: sets the difficulty from a sequence (useful to verify config flow)
    fn set_difficulty_interactive(&mut self) {
        let new_diff = Self::select_difficulty(&mut *self.printer);
        self.set_difficulty(new_diff);
    }

    fn set_players(&mut self) {
        self.printer.safe_print(Some(MessageKey::PlayersMenu), None, None, None, false, true, "PlayersMenu");

        loop {
            let input = self.printer.read_input().trim().to_uppercase();
            match input.as_str() {
                "1" => {
                    self.game = Game::Hangman;
                    self.printer.safe_print(
                        Some(MessageKey::ContinueMessage),
                        None,
                        None,
                        None,
                        false,
                        false,
                        "ContinueMessage",
                    );
                    break;
                }
                "2" => {
                    self.game = Game::Hangman2Players;
                    self.printer.safe_print(
                        Some(MessageKey::ContinueMessage),
                        None,
                        None,
                        None,
                        false,
                        false,
                        "ContinueMessage",
                    );
                    break;
                }
                _ => {
                    self.printer.safe_print(Some(MessageKey::InvalidOption), None, None, None, false, false, "InvalidOption");
                }
            }
        }
    }

    fn set_difficulty(&mut self, lives: u8) {
        // Ajustar valores y recomputar estado usando la API pública de Hangman.
        let current_word = self.hangman.word().to_string();
        self.hangman.set_initial_lives(lives);
        // change_word reconstruye `stages` en base a `initial_lives` y resetea el historial.
        self.hangman.change_word(current_word);

        if let Some(h2) = &mut self.hangman2 {
            let current_word2 = h2.word().to_string();
            h2.set_initial_lives(lives);
            h2.change_word(current_word2);
        }
    }

    /// Selección interactiva de dificultad (usa la `printer` proporcionada).
    fn select_difficulty(printer: &mut dyn GameUI) -> u8 {
        printer.safe_print(Some(MessageKey::DifficultyMenu), None, None, None, false, true, "DifficultyMenu");

        loop {
            let input = printer.read_input();
            let input_trim = input.trim();
            let difficulty = match input_trim {
                "1" => DifficultyLevel::Easy,
                "2" => DifficultyLevel::Medium,
                "3" => DifficultyLevel::Hard,
                "4" => DifficultyLevel::Insane,
                _ => {
                    printer.safe_print(Some(MessageKey::InvalidOption), None, None, None, false, false, "InvalidOption");
                    continue;
                }
            };

            return difficulty as u8;
        }
    }

    pub fn initial_lives(&self) -> u8 {
        self.hangman.initial_lives()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use crate::test_utils::DummyPrinter;

    use std::sync::Arc;

    use crate::test_utils::MockPrinter;

    #[test]
    fn set_difficulty_updates_hangman_instances() {
        let dir = tempdir().expect("tempdir");
        let boxed = Box::new(DummyPrinter);
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        // Default difficulty set in new_with_music_dir is based on DummyPrinter::read_input returning "1" -> Easy (8 lives)
        assert_eq!(gd.initial_lives(), 8);
        gd.set_difficulty(4);
        assert_eq!(gd.initial_lives(), 4);
    }

    #[test]
    fn set_players_switches_game() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty, then "2" to select two players
        let mp = MockPrinter::with_inputs(vec!["1", "2"]);
        let boxed: Box<dyn GameUI> = Box::new(mp);
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        // Now call set_players to consume the next input
        gd.set_players();
        assert_eq!(gd.game, Game::Hangman2Players);
    }

    #[test]
    fn main_menu_instructions_flow() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty, then "I" for instructions, then empty to exit
        let mp = MockPrinter::with_inputs(vec!["1", "I", ""]);
        let boxed: Box<dyn GameUI> = Box::new(mp);
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        gd.main_menu();
        // No panics and flow should have printed Instructions at least once
    }

    #[test]
    fn config_set_difficulty_changes_lives() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty; then in config: "5" -> enter difficulty select -> "2" -> set to Medium; then "6" to exit
        let mp = MockPrinter::with_inputs(vec!["1", "5", "2", "6"]);
        let boxed: Box<dyn GameUI> = Box::new(mp);
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        // call config directly (it will consume the queued inputs)
        gd.config();
        assert_eq!(gd.initial_lives(), 6);
    }
    #[test]
    fn ask_retry_clears_music_queue() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty, then answer "Y" to retry
        let mp = MockPrinter::with_inputs(vec!["1", "Y"]);
        let boxed: Box<dyn GameUI> = Box::new(Arc::clone(&mp));
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        // sanity: before calling ask_retry the queue may have songs
        let _q_before = gd.music_player.queue().join().expect("join");
        // Call ask_retry which should clear the queue
        let retry = gd.ask_retry();
        assert!(retry, "expected retry to be true");

        let q_after = gd.music_player.queue().join().expect("join");
        assert!(
            q_after.is_empty(),
            "expected music queue to be cleared on retry"
        );
    }

    #[test]
    fn finished_game_refreshes_screen() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty, then empty input for continue
        let mp = MockPrinter::with_inputs(vec!["1", ""]);
        let boxed: Box<dyn GameUI> = Box::new(Arc::clone(&mp));
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        // set a known word and call finished_game
        gd.hangman.change_word("ZZ".to_string());
        gd.finished_game(true);

        let outputs = mp.lock().unwrap().outputs.lock().unwrap().clone();
        // find the '<clear>' followed by the Congratulations message
        let mut found = false;
        for i in 0..outputs.len() - 1 {
            if outputs[i] == "<clear>" && outputs[i + 1].starts_with("Congratulations") {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "expected screen to be refreshed and show Congratulations"
        );
    }

    #[test]
    fn hid_refreshes_screen_for_easteregg() {
        let dir = tempdir().expect("tempdir");
        // initial "1" for difficulty, then the password 'HIDDEN', then empty
        let mp = MockPrinter::with_inputs(vec!["1", "HIDDEN", ""]);
        let boxed: Box<dyn GameUI> = Box::new(Arc::clone(&mp));
        let mut gd =
            GameData::new_with_music_dir(boxed, false, dir.path().to_str().unwrap()).expect("init");

        gd.hid();

        let outputs = mp.lock().unwrap().outputs.lock().unwrap().clone();
        // Check that there is a '<clear>' before the ASCII art (EASTEREGG)
        // Ensure a screen clear occurred and the art text was printed.
        assert!(
            outputs.iter().any(|s| s == "<clear>"),
            "expected a clear call"
        );
        // The ASCII art is a multi-line string; ensure at least one long block was printed.
        assert!(
            outputs.iter().any(|s| s.len() > 40),
            "expected the ASCII art to be printed"
        );
    }
}
