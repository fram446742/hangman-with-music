use std::io::{self, Result, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    lang::{Language, LanguageData},
    tools::random_color,
    messages::MessageKey,
    ui::GameUI,
};

#[derive(Debug, Clone)]
pub struct ColorfulWriterLang {
    pub language: Language,
    pub language_data: LanguageData,
}

#[derive(Debug)]
pub struct ColorfulWriter {
    stdout: StandardStream,
    color: Option<Color>,
    lang: ColorfulWriterLang,
}

impl ColorfulWriter {
    pub fn new(stdout: Option<StandardStream>) -> ColorfulWriter {
        let stdout = stdout.unwrap_or_else(|| StandardStream::stdout(ColorChoice::Auto));
        let language = Language::Global;
        let language_data = LanguageData::load(language);

        let lang = ColorfulWriterLang {
            language,
            language_data,
        };

        let mut instance = ColorfulWriter { stdout, color: Some(random_color()), lang };
        instance.set_language();

        instance
    }

    /// Writes colored text to the underlying `StandardStream`.
    pub fn write_colored(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()> {
        let mut color_spec = ColorSpec::new();
        color_spec.set_bold(bold);
        color_spec.set_fg(color.or(self.color));

        self.stdout.set_color(&color_spec)?;
        writeln!(self.stdout, "{}", text)?;
        self.stdout.reset()?;

        Ok(())
    }

    /// Lookup a message by `MessageKey`, format it with `extras` if provided
    /// and write it to the terminal with optional color/bold.
    pub fn write_message(
        &mut self,
        key: MessageKey,
        color: Option<Color>,
        extras: Option<&str>,
        bold: bool,
    ) -> Result<()> {
        let message = self
            .lang
            .language_data
            .get_message(key.as_str())
            .map_err(|e| io::Error::other(format!("Unable to get the message: {}", e)))?;

        let full_message = if let Some(extras) = extras {
            format!("{} {}", message, extras)
        } else {
            message
        };

        self.write_colored(&full_message, color, bold)
    }

    fn ask_language(&mut self) -> Language {
        let _ = self.write_message(MessageKey::LanguageMenu, Some(Color::White), None, false);
        let input = crate::console::read_input().trim().to_uppercase();

        match input.as_str() {
            "1" => Language::English,
            "2" => Language::Spanish,
            "3" => Language::French,
            "4" => Language::German,
            "5" => Language::Italian,
            "6" => Language::Portuguese,
            "7" => Language::Russian,
            "8" => Language::Chinese,
            "9" => Language::Japanese,
            _ => Language::English, // Default language if input is unknown
        }
    }

    /// Interactively ask the user for a language and reload messages.
    fn set_language(&mut self) {
        self.lang.language = self.ask_language();
        self.lang.language_data = LanguageData::load(self.lang.language);
    }

    /// Returns a reference to the current `LanguageData`.
    pub fn language_data(&self) -> &LanguageData {
        &self.lang.language_data
    }

    /// Trigger an interactive language change.
    pub fn change_language_interactive(&mut self) {
        self.set_language();
    }

    /// Set the theme color used as default when no explicit color is passed
    /// to `write_message`/`write_colored`.
    pub fn set_theme_color(&mut self, color: Option<Color>) {
        self.color = color;
    }
}

/// Implement `Console` trait for `ColorfulWriter` so it can be used where a
/// console is required in tests or other components.
impl crate::console::Console for ColorfulWriter {}

impl GameUI for ColorfulWriter {
    fn print_message(&mut self, key: MessageKey, color: Option<Color>, extras: Option<&str>, bold: bool) -> Result<()> {
        self.write_message(key, color, extras, bold)
    }

    fn print_colored(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()> {
        self.write_colored(text, color, bold)
    }

    fn read_input(&self) -> String {
        crate::console::read_input()
    }

    fn read_char(&self) -> Option<char> {
        crate::console::read_char()
    }

    fn read_pass(&self) -> String {
        crate::console::read_pass()
    }

    fn clear(&self) {
        crate::console::clear();
    }

    fn set_color(&mut self, color: Option<Color>) {
        self.set_theme_color(color);
    }

    fn change_language(&mut self) {
        self.change_language_interactive();
    }

    fn get_language_data(&self) -> &LanguageData {
        self.language_data()
    }
}