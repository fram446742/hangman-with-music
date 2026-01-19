use std::io::Result;
use termcolor::Color;

use crate::lang::LanguageData;
use crate::messages::MessageKey;

/// Trait que representa la interfaz de entrada/salida usada por la lógica del juego.
/// Permite reemplazar la UI (consola, pruebas, UI gráfica) sin tocar la lógica.
pub trait GameUI {
    fn print_message(&mut self, key: MessageKey, color: Option<Color>, extras: Option<&str>, bold: bool) -> Result<()>;
    fn print_colored(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()>;
    fn read_input(&self) -> String;
    fn read_char(&self) -> Option<char>;
    fn read_pass(&self) -> String;
    fn clear(&self);
    fn set_color(&mut self, color: Option<Color>);
    fn change_language(&mut self);
    fn get_language_data(&self) -> &LanguageData;
}
