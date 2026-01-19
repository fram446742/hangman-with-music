use std::io::Result;
use termcolor::Color;

use crate::lang::LanguageData;
use crate::messages::MessageKey;

pub type PrintingOptions = (Option<MessageKey>, Option<&'static str>, Option<Color>, Option<&'static str>, bool);

/// Trait que representa la interfaz de entrada/salida usada por la lógica del juego.
/// Permite reemplazar la UI (consola, pruebas, UI gráfica) sin tocar la lógica.
pub trait GameUI {
    fn print_message(
        &mut self,
        key: MessageKey,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
    ) -> Result<()>;
    fn print_colored(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()>;

    /// Centralized safe printing helper. Use `key` for localized messages or `text` for raw text printing.
    /// `screen` indicates whether to clear the screen before printing.
    fn safe_print(
        &mut self,
        key: Option<MessageKey>,
        text: Option<&str>,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
        screen: bool,
        context: &str,
    ) {
        if screen {
            self.clear();
        }

        let res = match (key, text) {
            (Some(k), None) => self.print_message(k, color, extras, bold),
            (None, Some(t)) => self.print_colored(t, color, bold),
            (Some(k), Some(t)) => self.print_message(k, color, Some(t), bold),
            (None, None) => return, // nothing to print
        };

        if let Err(e) = res {
            crate::logger::log_error(&format!("Failed to print {}", context), &format!("{}", e));
        }
    }

    /// Backwards-compatible convenience helpers implemented using `safe_print`.
    fn safe_print_message(
        &mut self,
        key: MessageKey,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
        context: &str,
    ) {
        self.safe_print(Some(key), None, color, extras, bold, false, context);
    }

    fn safe_print_colored(&mut self, text: &str, color: Option<Color>, bold: bool, context: &str) {
        self.safe_print(None, Some(text), color, None, bold, false, context);
    }

    fn safe_print_screen_message(
        &mut self,
        key: MessageKey,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
        context: &str,
    ) {
        self.safe_print(Some(key), None, color, extras, bold, true, context);
    }

    fn safe_print_colored_screen(
        &mut self,
        text: &str,
        color: Option<Color>,
        bold: bool,
        context: &str,
    ) {
        self.safe_print(None, Some(text), color, None, bold, true, context);
    }

    fn read_input(&self) -> String;
    fn read_char(&self) -> Option<char>;
    fn read_pass(&self) -> String;
    fn clear(&self);
    fn set_color(&mut self, color: Option<Color>);
    fn change_language(&mut self);
    fn get_language_data(&self) -> &LanguageData;
}
