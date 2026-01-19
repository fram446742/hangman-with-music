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

    pub fn print_colored_message(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()> {
        let mut color_spec = ColorSpec::new();
        color_spec.set_bold(bold);
        color_spec.set_fg(color.or(self.color));

        self.stdout.set_color(&color_spec)?;
        writeln!(self.stdout, "{}", text)?;
        self.stdout.reset()?;

        Ok(())
    }

    pub fn print_message(
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

        self.print_colored_message(&full_message, color, bold)
    }

    fn ask_language(&mut self) -> Language {
        let _ = self.print_message(MessageKey::LanguageMenu, Some(Color::White), None, false);
        let input = crate::tools::read_input().trim().to_uppercase();

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

    fn set_language(&mut self) {
        self.lang.language = self.ask_language();
        self.lang.language_data = LanguageData::load(self.lang.language);
    }

    // fn get_language(&self) -> Language {
    //     self.lang.language
    // }

    pub fn get_language_data(&self) -> &LanguageData {
        &self.lang.language_data
    }

    pub fn change_language(&mut self) {
        self.set_language();
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }
}

impl GameUI for ColorfulWriter {
    fn print_message(&mut self, key: MessageKey, color: Option<Color>, extras: Option<&str>, bold: bool) -> Result<()> {
        self.print_message(key, color, extras, bold)
    }

    fn print_colored(&mut self, text: &str, color: Option<Color>, bold: bool) -> Result<()> {
        self.print_colored_message(text, color, bold)
    }

    fn read_input(&self) -> String {
        crate::tools::read_input()
    }

    fn read_char(&self) -> Option<char> {
        crate::tools::read_char()
    }

    fn read_pass(&self) -> String {
        crate::tools::read_pass()
    }

    fn clear(&self) {
        crate::tools::clear();
    }

    fn set_color(&mut self, color: Option<Color>) {
        self.set_color(color);
    }

    fn change_language(&mut self) {
        self.change_language();
    }

    fn get_language_data(&self) -> &LanguageData {
        self.get_language_data()
    }
}
