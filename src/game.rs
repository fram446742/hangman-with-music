use crate::{
    consts::{ EASTEREGG, EASTEREGG2 },
    hangman::Hangman,
    player::MusicPlayer,
    printer::ColorfulWriter,
    tools::{ clear, random_color, read_char, read_input, read_pass },
};
use rand::Rng;
use std::{ process::exit, sync::RwLock };

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

#[allow(dead_code)]
pub struct GameData {
    pub game: Game,
    pub hangman: Option<Box<Hangman>>,
    pub hangman2: Option<Box<Hangman>>,
    pub difficulty: u8,
    pub music_player: MusicPlayer,
    pub printer: ColorfulWriter,
}

impl GameData {
    pub fn new(printer: RwLock<ColorfulWriter>, two_players: bool) -> Self {
        let printer = printer.into_inner().unwrap();
        let mut instance = Self {
            game: if two_players {
                Game::Hangman2Players
            } else {
                Game::Hangman
            },
            hangman: None,
            hangman2: None,
            difficulty: 0, // Temporary value
            music_player: MusicPlayer::new(),
            printer,
        };

        instance.difficulty = instance.select_difficulty();

        let language_data = instance.printer.get_language_data().clone();


        let (hangman, hangman2) = match two_players {
            false => (Box::new(Hangman::new(instance.difficulty, language_data.clone())), None),
            true => (
                Box::new(Hangman::new(instance.difficulty, language_data.clone())),
                Some(Box::new(Hangman::new(instance.difficulty, language_data.clone())))
            ),
        };

        instance.music_player.init();
        instance.music_player.start_music();

        // Assign Hangman instances to the struct
        instance.hangman = Some(hangman);
        instance.hangman2 = hangman2;

        instance.set_difficulty(instance.difficulty);
        instance
    }

    // FIXME
    fn finished_game(&mut self, won: bool) {
        match won {
            true => {
                let _ = self.printer.print_message(
                    "Congratulations",
                    None,
                    Some(self.hangman.as_ref().unwrap().word.clone()),
                    false
                );
            }
            _ => {
                let _ = self.printer.print_message(
                    "GameOver",
                    None,
                    Some(self.hangman.as_ref().unwrap().word.clone()),
                    false
                );
            }
        }
    }
    
    pub fn run_game(&mut self) {
        if self.game == Game::Hangman {
            self.main_menu();
            clear();


            let hangman = self.hangman.as_mut().unwrap();
            hangman.display(None, &mut self.printer);

            loop {
                let input = read_char();
                if let Some(guess) = input {

                    if self.hangman.as_mut().unwrap().guess(guess, &mut self.printer) {
                        // let _ = self.printer.print_message("AcceptedLetter", None, None::<String>, false);
                    }
                } else {
                    // let _ = self.printer.print_message("InvalidCharacter", None, None::<String>, false);
                    continue;
                }

                if self.hangman.as_mut().unwrap().is_lost() {
                    self.finished_game(false);
                    // game_over(hangman, stdout);
                    break;
                } else if self.hangman.as_mut().unwrap().is_won() {
                    self.finished_game(true);
                    // game_finish(hangman, stdout);
                    break;
                }
            }
        } else if self.game == Game::Hangman2Players {
            todo!("Two players mode is not implemented yet");
            // return None;
        } else {
            panic!("Invalid game mode");
        }

        //
    }

    // FIXME: fix the menu
    pub fn main_menu(&mut self) {
        loop {
            clear();
            let _ = self.printer.print_message("WelcomeBanner", None, None::<String>, false);
            let _ = self.printer.print_message("StartMessage", None, None::<String>, false);

            let input = read_input().trim().to_uppercase();
            match input.as_str() {
                "I" => {
                    clear();
                    let _ = self.printer.print_message("Instructions", None, None::<String>, false);
                    let _ = self.printer.print_message("ContinueMessage", None, None::<String>, false);
                    read_input();
                }
                "S" | "A" => {
                    clear();
                    self.config();
                }
                "E" => {
                    exit(0);
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn config(&mut self) {
        loop {
            clear();
            let _ = self.printer.print_message("SettingsMenu", None, None::<String>, false);
            let input = read_input().trim().to_uppercase();
            // Check the user input and call the corresponding function
            match input.as_str() {
                "1" => {
                    self.printer.set_color(Some(random_color()));
                }
                "2" => {
                    self.music_player.toggle_music();
                }
                "3" => {
                    self.set_players();
                }
                "4" => {
                    self.printer.change_language();
                    self.hangman
                        .as_mut()
                        .unwrap()
                        .change_word(
                            self.printer
                                .get_language_data()
                                .get_random_word()
                                .unwrap_or(String::from("TestWord"))
                        );
                }
                "5" => {
                    self.select_difficulty();
                }
                "EASTEREGG" => {
                    self.hid();
                }
                "6" | _ => {
                    // clear();
                    break;
                }
            }
        }
        // main_menu(stdout, hangman, music_player);
    }
    pub fn hid(&mut self) {
        let _ = self.printer.print_message("InsertPassword", None, None::<String>, false);
        let pass = read_pass();
        match
            pass == "HIDDEN" ||
            pass == "hidden" ||
            pass == "Hidden" ||
            pass == "OCULTO" ||
            pass == "oculto" ||
            pass == "Oculto"
        {
            true => {
                let _ = self.printer.print_message("AccessGranted", None, None::<String>, false);
                // NOTE: There was an easter egg here, but it's gone now, weird...
                let _ = self.printer.print_message("EasterEgg1", None, None::<String>, false);

                let _ = self.printer.print_colored(EASTEREGG, None, false);
                // print_colored_text(stdout, get_message(21), color);
                let _ = self.printer.print_colored(
                    &format!(
                        "{} {}",
                        self.printer
                            .get_language_data()
                            .get_message("ContinueMessage")
                            .unwrap_or(
                                String::from("**Missing message for key: ContinueMessage**")
                            ),
                        String::from("?")
                    ),
                    None,
                    false
                );

                read_input();
                if rand::thread_rng().gen_range(0..=5) == 5 {
                    let _ = self.printer.print_colored(EASTEREGG2, None, false);
                    // NOTE: There was a second easter egg here, but it's gone now, wonder where it went...
                    let _ = self.printer.print_message("EasterEgg2", None, None::<String>, false);

                    read_input();
                }
            }
            false => {
                let _ = self.printer.print_message("AccessDenied", None, None::<String>, false);
                let _ = self.printer.print_message("ContinueMessage", None, None::<String>, false);

                read_input();
            }
        }
    }

    #[allow(dead_code)]
    fn set_players(&mut self) {
        clear();
        let _ = self.printer.print_message("PlayersMenu", None, None::<String>, false);

        loop {
            let input = read_input().trim().to_uppercase();
            match input.as_str() {
                // FIXME: make this work
                "1" => {
                    // *NUM_PLAYERS.lock().unwrap() = 1;
                    panic!("Not implemented");
                    // break;
                }
                "2" => {
                    panic!("Not implemented");
                    // *NUM_PLAYERS.lock().unwrap() = 2;
                    // break;
                }
                _ => {
                    let _ = self.printer.print_message("InvalidOption", None, None::<String>, false);
                }
            }
        }
    }

    // FIXME: make this work
    fn set_difficulty(&mut self, lives: u8) {
        self.hangman.as_mut().unwrap().lives = lives;
        self.hangman.as_mut().unwrap().initial_lives = lives;
        // let mut dif = DIFFICULTY.lock().unwrap();
        // *dif = lives;
    }

    fn select_difficulty(&mut self) -> u8 {
        clear();
        let _ = self.printer.print_message("DifficultyMenu", None, None::<String>, false);

        loop {
            let read = read_input();
            let input = read.trim();
            // Check if the input is a number between 1 and 4 and set the lives accordingly
            let difficulty = match input {
                "1" => DifficultyLevel::Easy,
                "2" => DifficultyLevel::Medium,
                "3" => DifficultyLevel::Hard,
                "4" => DifficultyLevel::Insane,
                _ => {
                    let _ = self.printer.print_message("InvalidOption", None, None::<String>, false);
                    continue;
                }
            };

            let lives = difficulty as u8;
            return lives;
            // break;
        }
    }
}
