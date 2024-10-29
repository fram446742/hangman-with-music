// use std::collections::HashMap;

// #[derive(PartialEq, Eq, Hash, Debug, Clone)]
// pub enum Language {
//     Spanish,
//     English,
//     French,
//     German,
//     Italian,
//     Portuguese,
//     Russian,
//     Chinese,
//     Japanese,
//     Global,
// }

// #[derive(PartialEq, Eq, Hash, Debug, Clone)]
// pub enum MessageId {
//     StartMessage,
//     ContinueMessage,
//     EnterWord,
//     WordTooShort,
//     WordTooLong,
//     EasterEgg1,
//     PromptLetter,
//     InvalidCharacter,
//     LetterAlreadyUsed,
//     IncorrectLetter,
//     AcceptedLetter,
//     GameOver,
//     Congratulations,
//     Lives,
//     LineReadError,
//     InvalidOption,
//     RetryPrompt,
//     InsertPassword,
//     AccessGranted,
//     AccessDenied,
//     ContinuePrompt,
//     WelcomeBanner,
//     Instructions,
//     SettingsMenu,
//     LanguageMenu,
//     DifficultyMenu,
//     EasterEgg2,
//     PlayersMenu,
//     ChoosePlayers,
//     WordDisplay,
//     GuessedLetters,
//     // Add more message IDs as needed
// }

// // Define the Messages struct with a HashMap to store the messages
// #[derive(Debug, Clone)]
// pub struct Messages {
//     language_data: HashMap<(Language, MessageId), &'static str>,
// }

// impl Messages {
//     pub fn new() -> Self {
//         let mut language_data = HashMap::new();

//         // Insert Spanish messages
//         language_data.insert((Language::Spanish, MessageId::StartMessage), "Presione Enter para iniciar el juego, I para ver las instrucciones, A para entrar en los ajustes o E para salir...");
//         language_data.insert(
//             (Language::Spanish, MessageId::ContinueMessage),
//             "Presione Enter para continuar...",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::EnterWord),
//             "Ingrese la palabra a adivinar: ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::WordTooShort),
//             "La palabra debe tener al menos 2 letras.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::WordTooLong),
//             "La palabra debe tener menos de 15 letras.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::EasterEgg1),
//             "¡Felicidades! Has encontrado el huevo de pascua.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::PromptLetter),
//             "Prueba una letra",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::InvalidCharacter),
//             "Por favor, ingrese un carácter válido.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::LetterAlreadyUsed),
//             "Letra ya usada.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::IncorrectLetter),
//             "Letra incorrecta.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::AcceptedLetter),
//             "Letra aceptada.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::GameOver),
//             "G A M E   O V E R!!!\nLa palabra era: ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::Congratulations),
//             "F E L I C I D A D E S!!!\nHas adivinado la palabra: ",
//         );
//         language_data.insert((Language::Spanish, MessageId::Lives), "Vidas: ");
//         language_data.insert(
//             (Language::Spanish, MessageId::LineReadError),
//             "No se pudo leer la línea.",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::InvalidOption),
//             "Por favor, seleccione una opción válida",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::RetryPrompt),
//             "Desea volver a intentarlo? (s/n): ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::InsertPassword),
//             "
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡀⠀⠀⢀⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣤⣤⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢀⣀⣠⠀⣶⣤⣄⣉⣉⣉⣉⣠⣤⣶⠀⣄⣀⡀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⣶⣾⣿⣿⣿⣿⣦⣄⣉⣙⣛⣛⣛⣛⣋⣉⣠⣴⣿⣿⣿⣿⣷⣶⠀⠀⠀
// ⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠛⠛⠉⠉⠁⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⣆⠀⠀⠀⢠⡄⠀⠀⠀⣰⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⢀⣠⣶⣾⣿⡆⠸⣿⣶⣶⣾⣿⣿⣷⣶⣶⣿⠇⢰⣿⣷⣶⣄⡀⠀⠀⠀
// ⠀⠀⠺⠿⣿⣿⣿⣿⣿⣄⠙⢿⣿⣿⣿⣿⣿⣿⡿⠋⣠⣿⣿⣿⣿⣿⠿⠗⠀⠀
// ⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣷⡄⠈⠙⠛⠛⠋⠁⢠⣾⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⣀⣤⣬⣿⣿⣿⣇⠐⣿⣿⣿⣿⠂⣸⣿⣿⣿⣥⣤⣀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠘⠻⠿⠿⢿⣿⣿⣿⣧⠈⠿⠿⠁⣼⣿⣿⣿⡿⠿⠿⠟⠃⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⠀⣶⣦⠀⡿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//     Inserte contraseña: ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::AccessGranted),
//             "Acceso concedido",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::AccessDenied),
//             "Acceso denegado",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::EasterEgg2),
//             "¡Felicidades! Has encontrado el segundo huevo de pascua. Que suerte tienes ;) 🍀",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::WelcomeBanner),
//             "
// *******************************************************
// *                                                     *
// *              ¡BIENVENIDO AL JUEGO DEL               *
// *                      AHORCADO!                      *
// *                     ________                        *
// *                     |/      |                       *
// *                     |      (_)                      *
// *                     |      \\|/                      *
// *                     |       |                       *
// *                     |      / \\                      *
// *                     |                               *
// *                    _|___                            *
// *                                                     *
// *******************************************************
// ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::Instructions),
//             "
// **********************************************************************************************
// *                                      Instrucciones:                                        *
// *                                                                                            *
// * - Debes adivinar la palabra oculta.                                                        *
// * - Tienes un número limitado de vidas (por defecto es 4, puedes cambiarlo en ajustes).      *
// * - Cada vez que ingreses una letra incorrecta, perderás una vida.                           *
// * - Si adivinas la palabra sin perder todas tus vidas, ¡felicidades!                         *
// * - Si te quedas sin vidas, pierdes.                                                         *
// * - ¡Diviértete!                                                                             *
// *                                                                                            *
// * El juego aún está en desarrollo, así que ignora cualquier error (^_^')                     *
// *                                                                                            *
// * Recuerda explorar todas las partes del juego, a veces hay sorpresas ocultas...             *
// * Como un 'easteregg'... esperando ser descubierto.                                          *
// **********************************************************************************************
// ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::SettingsMenu),
//             "
// **************************************************************************
// *                                 Ajustes:                               *
// *                                                                        *
// * - Presiona 1 para cambiar el color.                                    *
// * - Presiona 2 para alternar la musica.                                  *
// * - Presiona 3 para cambiar el numero de jugadores.                      *
// * - Presiona 4 para cambiar el idioma.                                   *
// * - Presiona 5 para cambiar la dificultad.                               *
// * - Presiona cualquier otra cosa para para volver al menu principal.     *
// * - Me parece que puede existir algo 'oculto' en alguna parte...         *
// **************************************************************************
// ",
//         );
//         language_data.insert(
//             (Language::Global, MessageId::LanguageMenu),
//             "
// ************************************************************************
// *                                 🌍🌐:                               *
// *                                                                      *
// * 1.  Español (ES).                                                    *
// * 2.  English (EN).                                                    *
// ************************************************************************
// ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::DifficultyMenu),
//             "
// **********************************
// *                                 *
// *   Seleccione la dificultad:     *
// *                                 *
// *   1. Facil (6 vidas)   (^o^)    *
// *   2. Medio (4 vidas)   (·_·')   *
// *   3. Dificil (2 vidas)  O_O     *
// *   4. Imposible (1 vidas) x_x    *
// *                                 *
// **********************************
// ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::PlayersMenu),
//             "
// ************************************************************************
// *                                Jugadores:                            *
// *                                                                      *
// * 1.  Un solo jugador.                                                 *
// * 2.  2 Jugadores (no implementado, rompera el juego).                 *
// ************************************************************************
// ",
//         );
//         language_data.insert(
//             (Language::Spanish, MessageId::ChoosePlayers),
//             "Por favor elige número de jugadores",
//         );
//         language_data.insert((Language::Spanish, MessageId::WordDisplay), "Palabra: ");
//         language_data.insert(
//             (Language::Spanish, MessageId::GuessedLetters),
//             "Letras usadas: ",
//         );

//         // Insert English messages
//         language_data.insert((Language::English, MessageId::StartMessage), "Press Enter to start the game, I to see the instructions, S to enter the settings, or E to exit...");
//         language_data.insert(
//             (Language::English, MessageId::ContinueMessage),
//             "Press Enter to continue...",
//         );
//         language_data.insert(
//             (Language::English, MessageId::EnterWord),
//             "Enter the word to guess:",
//         );
//         language_data.insert(
//             (Language::English, MessageId::WordTooShort),
//             "The word must have at least 2 letters.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::WordTooLong),
//             "The word must have less than 15 letters.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::EasterEgg1),
//             "Congratulations! You have found the Easter egg.",
//         );
//         language_data.insert((Language::English, MessageId::PromptLetter), "Try a letter");
//         language_data.insert(
//             (Language::English, MessageId::InvalidCharacter),
//             "Please enter a valid character.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::LetterAlreadyUsed),
//             "Letter already used.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::IncorrectLetter),
//             "Incorrect letter.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::AcceptedLetter),
//             "Accepted letter.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::GameOver),
//             "G A M E   O V E R!!!\nThe word was: ",
//         );
//         language_data.insert(
//             (Language::English, MessageId::Congratulations),
//             "C O N G R A T U L A T I O N S!!!\nYou have guessed the word: ",
//         );
//         language_data.insert((Language::English, MessageId::Lives), "Lives: ");
//         language_data.insert(
//             (Language::English, MessageId::LineReadError),
//             "The line could not be read.",
//         );
//         language_data.insert(
//             (Language::English, MessageId::InvalidOption),
//             "Please select a valid option",
//         );
//         language_data.insert(
//             (Language::English, MessageId::RetryPrompt),
//             "Do you want to try again? (y/n): ",
//         );
//         language_data.insert(
//             (Language::English, MessageId::InsertPassword),
//             "
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡀⠀⠀⢀⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣤⣤⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⢀⣀⣠⠀⣶⣤⣄⣉⣉⣉⣉⣠⣤⣶⠀⣄⣀⡀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⣶⣾⣿⣿⣿⣿⣦⣄⣉⣙⣛⣛⣛⣛⣋⣉⣠⣴⣿⣿⣿⣿⣷⣶⠀⠀⠀
// ⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠛⠛⠉⠉⠁⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⣆⠀⠀⠀⢠⡄⠀⠀⠀⣰⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⢀⣠⣶⣾⣿⡆⠸⣿⣶⣶⣾⣿⣿⣷⣶⣶⣿⠇⢰⣿⣷⣶⣄⡀⠀⠀⠀
// ⠀⠀⠺⠿⣿⣿⣿⣿⣿⣄⠙⢿⣿⣿⣿⣿⣿⣿⡿⠋⣠⣿⣿⣿⣿⣿⠿⠗⠀⠀
// ⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣷⡄⠈⠙⠛⠛⠋⠁⢠⣾⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⣀⣤⣬⣿⣿⣿⣇⠐⣿⣿⣿⣿⠂⣸⣿⣿⣿⣥⣤⣀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠘⠻⠿⠿⢿⣿⣿⣿⣧⠈⠿⠿⠁⣼⣿⣿⣿⡿⠿⠿⠟⠃⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⠀⣶⣦⠀⡿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//     Insert password: ",
//         );
//         language_data.insert(
//             (Language::English, MessageId::AccessGranted),
//             "Access granted",
//         );
//         language_data.insert(
//             (Language::English, MessageId::AccessDenied),
//             "Access denied",
//         );
//         language_data.insert(
//             (Language::English, MessageId::EasterEgg2),
//             "Congratulations! You have found the second Easter egg. You are so lucky ;) 🍀",
//         );
//         language_data.insert(
//             (Language::English, MessageId::WelcomeBanner),
//             "
// *******************************************************
// *                                                     *
// *                WELCOME TO THE HANGMAN               *
// *                        GAME!                        *
// *                     ________                        *
// *                     |/      |                       *
// *                     |      (_)                      *
// *                     |      \\|/                      *
// *                     |       |                       *
// *                     |      / \\                      *
// *                     |                               *
// *                    _|___                            *
// *                                                     *
// *******************************************************
// ",
//         );
//         language_data.insert(
//             (Language::English, MessageId::Instructions),
//             "
// **********************************************************************************************
// *                                       Instructions:                                        *
// *                                                                                            *
// * - You must guess the hidden word.                                                          *
// * - You have a limited number of lives (default is 4, you can change it in settings).        *
// * - Every time you enter an incorrect letter, you will lose a life.                          *
// * - If you guess the word without losing all your lives, congrats!                           *
// * - If you run out of lives, you lose.                                                       *
// * - Have fun!                                                                                *
// *                                                                                            *
// * The game is still under development, so please ignore any bugs (^_^')                      *
// *                                                                                            *
// * Remember to explore all parts of the game, sometimes there are hidden surprises...         *
// * Like an 'easteregg'... just waiting to be found.                                           *
// **********************************************************************************************
// ",
//         );

//         language_data.insert(
//             (Language::English, MessageId::SettingsMenu),
//             "
// **************************************************************************
// *                                 Settings:                              *
// *                                                                        *
// * - Press 1 to change the color.                                         *
// * - Press 2 to toggle the music.                                         *
// * - Press 3 to change the number of players.                             *
// * - Press 4 to change the language.                                      *
// * - Press 5 to change the difficulty.                                    *
// * - Press anything else to return to the main menu.                      *
// * - It seems to me that there may be something 'hidden' somewhere...     *
// **************************************************************************
// ",
//         );

//         language_data.insert(
//             (Language::English, MessageId::DifficultyMenu),
//             "
// **********************************
// *                                 *
// *   Select the difficulty:        *
// *                                 *
// *   1. Easy (6 lives)     (^o^)   *
// *   2. Medium (4 lives)   (·_·')  *
// *   3. Hard (2 lives)      O_O    *
// *   4. Impossible (1 lives) x_x   *
// *                                 *
// **********************************
// ",
//         );

//         language_data.insert(
//             (Language::English, MessageId::PlayersMenu),
//             "
//     ************************************************************************
//     *                                Players:                              *
//     *                                                                      *
//     * 1.  Single player.                                                   *
//     * 2.  2 Players (Not implemented, will break the game).                *
//     ************************************************************************
//     ",
//         );

//         language_data.insert(
//             (Language::English, MessageId::ChoosePlayers),
//             "Please choose number of players",
//         );
//         language_data.insert((Language::English, MessageId::WordDisplay), "Word: ");
//         language_data.insert(
//             (Language::English, MessageId::GuessedLetters),
//             "Guessed letters: ",
//         );

//         Self { language_data }
//     }

//     pub fn get_message(
//         &self,
//         language: &Language,
//         id: MessageId,
//     ) -> Result<&'static str, MessageError> {
//         self.language_data
//             .get(&(language, id))
//             .map(|&msg| msg)
//             .ok_or(MessageError::NotFound)
//     }
// }