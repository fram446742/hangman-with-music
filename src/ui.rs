use std::io::Result;
use termcolor::Color;

use crate::lang::LanguageData;
use crate::messages::MessageKey;

/// Options for a centralized safe print call. Using a struct reduces argument count and simplifies extensions.
pub struct SafePrintOptions {
    pub key: Option<MessageKey>,
    pub text: Option<String>,
    pub color: Option<Color>,
    pub extras: Option<String>,
    pub bold: bool,
    pub screen: bool,
    pub context: String,
}

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

    /// Centralized safe printing helper that takes a single options struct.
    fn safe_print(&mut self, opts: SafePrintOptions) {
        if opts.screen {
            self.clear();
        }

        let res = match (opts.key, opts.text.as_deref()) {
            (Some(k), None) => self.print_message(k, opts.color, opts.extras.as_deref(), opts.bold),
            (None, Some(t)) => self.print_colored(t, opts.color, opts.bold),
            (Some(k), Some(t)) => self.print_message(k, opts.color, Some(t), opts.bold),
            (None, None) => return,
        };

        if let Err(e) = res {
            crate::logger::log_error(
                &format!("Failed to print {}", opts.context),
                &format!("{}", e),
            );
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
        self.safe_print(SafePrintOptions {
            key: Some(key),
            text: None,
            color,
            extras: extras.map(|s| s.to_string()),
            bold,
            screen: false,
            context: context.to_string(),
        });
    }

    fn safe_print_colored(&mut self, text: &str, color: Option<Color>, bold: bool, context: &str) {
        self.safe_print(SafePrintOptions {
            key: None,
            text: Some(text.to_string()),
            color,
            extras: None,
            bold,
            screen: false,
            context: context.to_string(),
        });
    }

    fn safe_print_screen_message(
        &mut self,
        key: MessageKey,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
        context: &str,
    ) {
        self.safe_print(SafePrintOptions {
            key: Some(key),
            text: None,
            color,
            extras: extras.map(|s| s.to_string()),
            bold,
            screen: true,
            context: context.to_string(),
        });
    }

    fn safe_print_colored_screen(
        &mut self,
        text: &str,
        color: Option<Color>,
        bold: bool,
        context: &str,
    ) {
        self.safe_print(SafePrintOptions {
            key: None,
            text: Some(text.to_string()),
            color,
            extras: None,
            bold,
            screen: true,
            context: context.to_string(),
        });
    }

    fn read_input(&self) -> String;
    fn read_char(&self) -> Option<char>;
    fn read_pass(&self) -> String;
    fn clear(&self);
    fn set_color(&mut self, color: Option<Color>);
    fn change_language(&mut self);
    fn get_language_data(&self) -> &LanguageData;
}
