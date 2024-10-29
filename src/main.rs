use game::GameData;
use std::process::exit;
use termcolor::{ ColorChoice, StandardStream };
use tools::{ clear, read_input };
mod consts;
mod game;
mod hangman;
mod lang;
mod player;
mod tools;
mod printer;

#[allow(unused)]
fn game_loop() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    loop {
        // Implement choosing another game

        // FIXME: Implement the game selection menu and languages not dependant on the gamedata

        clear();

        let mut printer = printer::ColorfulWriter::new(None);

        let mut hangman = GameData::new(printer, false);

        // FIXME: Send printer to the game data
        hangman.run_game();
        hangman.printer.print_message("RetryPrompt", None, None::<String>, false);

        let input = read_input().trim().to_uppercase();

        if input != "S" && input != "Y" {
            break; // Exit the loop if the user does not want to restart
        }
    }

    // // Exit the game cleanly
    exit(0);
}

// fn game_chosen(game: Game) {
//     let printer = printer::ColorfulWriter::new(None);
//     match game {
//         Game::Hangman => {
//             let mut hangman = GameData::new(printer, false);
//             hangman.run_game();
//         }
//         Game::Hangman2Players => {
//             let mut hangman2players = GameData::new(printer, true);
//             hangman2players.run_game();
//         }
//     }
// }

fn main() {
    game_loop(); // Start the game loop
}
