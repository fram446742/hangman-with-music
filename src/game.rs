use anyhow::Context;
use crate::{
    consts::{EASTEREGG, EASTEREGG2},
    hangman::Hangman,
    lang::LanguageData,
    messages::MessageKey,
    player::MusicPlayer,
    tools::random_color,
    ui::GameUI,
};
use rand::Rng;
use std::process::exit;
use anyhow::Result;

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

/// Contenedor principal del estado y la lógica del juego.
pub struct GameData {
    pub game: Game,
    pub hangman: Box<Hangman>,
    pub hangman2: Option<Box<Hangman>>,
    pub music_player: MusicPlayer,
    pub printer: Box<dyn GameUI>,
}

impl GameData {
    /// Crea una nueva instancia preguntando la dificultad y preparando música y palabra.
    pub fn new(printer: Box<dyn GameUI>, two_players: bool) -> Result<Self> {
        Self::new_with_music_dir(printer, two_players, "./music")
    }

    /// Create a new GameData instance but allow specifying the music directory (useful for tests)
    pub fn new_with_music_dir(mut printer: Box<dyn GameUI>, two_players: bool, music_dir: &str) -> Result<Self> {
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
        music.init_in(music_dir).context("initializing music player failed")?;
        music.start_music();

        let mut instance = Self {
            game: if two_players { Game::Hangman2Players } else { Game::Hangman },
            hangman,
            hangman2,
            music_player: music,
            printer,
        };

        instance.set_difficulty(difficulty);
        Ok(instance)
    }

    fn finished_game(&mut self, won: bool) {
        if won {
            let _ = self.printer.print_message(
                MessageKey::Congratulations,
                None,
                Some(&self.hangman.word),
                false,
            );
        } else {
            let _ = self.printer.print_message(
                MessageKey::GameOver,
                None,
                Some(&self.hangman.word),
                false,
            );
        }

        let _ = self.printer.print_message(MessageKey::ContinueMessage, None, None, false);
        // esperar a que el usuario presione Enter
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
                            let _ = self.printer.print_message(MessageKey::InvalidCharacter, None, None, false);
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
                let _ = self.printer.print_message(MessageKey::InvalidOption, None, None, false);
            }
        }

        Ok(())
    }

    /// Run the application loop at the module level.
    pub fn run_app() -> anyhow::Result<()> {
        loop {
            let writer = crate::printer::ColorfulWriter::new(None);
            let boxed_printer: Box<dyn crate::ui::GameUI> = Box::new(writer);

            let mut gd = GameData::new(boxed_printer, false).context("Failed to initialize game data")?;
            gd.run()?;
            let _ = gd.printer.print_message(crate::messages::MessageKey::RetryPrompt, None, None, false);
            let input = gd.printer.read_input().trim().to_uppercase();
            if input != "S" && input != "Y" {
                break;
            }
        }

        Ok(())
    }

    pub fn main_menu(&mut self) {
        loop {
            self.printer.clear();
            let _ = self.printer.print_message(MessageKey::WelcomeBanner, None, None, false);
            let _ = self.printer.print_message(MessageKey::StartMessage, None, None, false);

            let input = self.printer.read_input().trim().to_uppercase();
            match input.as_str() {
                "I" => {
                    self.printer.clear();
                    let _ = self.printer.print_message(MessageKey::Instructions, None, None, false);
                    let _ = self.printer.print_message(MessageKey::ContinueMessage, None, None, false);
                    let _ = self.printer.read_input();
                }
                "S" | "A" => {
                    self.printer.clear();
                    self.config();
                }
                "E" => exit(0),
                _ => break,
            }
        }
    }

    fn config(&mut self) {
        loop {
            self.printer.clear();
            let _ = self.printer.print_message(MessageKey::SettingsMenu, None, None, false);
            let input = self.printer.read_input().trim().to_uppercase();

            match input.as_str() {
                "1" => self.printer.set_color(Some(random_color())),
                "2" => self.music_player.toggle_music(),
                "3" => self.set_players(),
                "4" => {
                    self.printer.change_language();
                    let new_word = self.printer.get_language_data().get_random_word().unwrap_or_else(|| String::from("TestWord"));
                    self.hangman.change_word(new_word);
                }
                "5" => {
                    let new_diff = Self::select_difficulty(&mut *self.printer);
                    self.set_difficulty(new_diff);
                }
                "EASTEREGG" => self.hid(),
                "6" => break,
                _ => {
                    let _ = self.printer.print_message(MessageKey::InvalidOption, None, None, false);
                }
            }
        }
    }

    pub fn hid(&mut self) {
        let _ = self.printer.print_message(MessageKey::InsertPassword, None, None, false);
        let pass = self.printer.read_pass();
        let pass_ok = matches!(pass.as_str(), "HIDDEN" | "hidden" | "Hidden" | "OCULTO" | "oculto" | "Oculto");

        if pass_ok {
            let _ = self.printer.print_message(MessageKey::AccessGranted, None, None, false);
            let _ = self.printer.print_message(MessageKey::EasterEgg1, None, None, false);
            let _ = self.printer.print_colored(EASTEREGG, None, false);

            let continue_msg = self
                .printer
                .get_language_data()
                .get_message(MessageKey::ContinueMessage.as_str())
                .unwrap_or_else(|_| String::from("**Missing message for key: ContinueMessage**"));

            let _ = self.printer.print_colored(&format!("{} {}", continue_msg, "?"), None, false);
            let _ = self.printer.read_input();

            if rand::rng().random_range(0..=5) == 5 {
                let _ = self.printer.print_colored(EASTEREGG2, None, false);
                let _ = self.printer.print_message(MessageKey::EasterEgg2, None, None, false);
                let _ = self.printer.read_input();
            }
        } else {
            let _ = self.printer.print_message(MessageKey::AccessDenied, None, None, false);
            let _ = self.printer.print_message(MessageKey::ContinueMessage, None, None, false);
            let _ = self.printer.read_input();
        }
    }

    fn set_players(&mut self) {
        self.printer.clear();
        let _ = self.printer.print_message(MessageKey::PlayersMenu, None, None, false);

        loop {
            let input = self.printer.read_input().trim().to_uppercase();
            match input.as_str() {
                "1" => {
                    let _ = self.printer.print_message(MessageKey::InvalidOption, None, None, false);
                }
                "2" => {
                    let _ = self.printer.print_message(MessageKey::InvalidOption, None, None, false);
                }
                _ => {
                    let _ = self.printer.print_message(MessageKey::InvalidOption, None, None, false);
                }
            }
        }
    }

    fn set_difficulty(&mut self, lives: u8) {
        // Ajustar valores y recomputar estado usando la API pública de Hangman.
        self.hangman.initial_lives = lives;
        self.hangman.lives = lives;
        let current_word = self.hangman.word.clone();
        // change_word reconstruye `stages` en base a `initial_lives` y resetea el historial.
        self.hangman.change_word(current_word);

        if let Some(h2) = &mut self.hangman2 {
            h2.initial_lives = lives;
            h2.lives = lives;
            let current_word2 = h2.word.clone();
            h2.change_word(current_word2);
        }
    }

    /// Selección interactiva de dificultad (usa la `printer` proporcionada).
    fn select_difficulty(printer: &mut dyn GameUI) -> u8 {
        printer.clear();
        let _ = printer.print_message(MessageKey::DifficultyMenu, None, None, false);

        loop {
            let input = printer.read_input();
            let input_trim = input.trim();
            let difficulty = match input_trim {
                "1" => DifficultyLevel::Easy,
                "2" => DifficultyLevel::Medium,
                "3" => DifficultyLevel::Hard,
                "4" => DifficultyLevel::Insane,
                _ => {
                    let _ = printer.print_message(MessageKey::InvalidOption, None, None, false);
                    continue;
                }
            };

            return difficulty as u8;
        }
    }
}
