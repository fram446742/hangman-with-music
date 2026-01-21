#![cfg(test)]
use crate::lang::Language;
use crate::lang::LanguageData;
use crate::ui::GameUI;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct DummyPrinter;
impl GameUI for DummyPrinter {
    fn print_message(
        &mut self,
        _key: crate::messages::MessageKey,
        _color: Option<termcolor::Color>,
        _extras: Option<&str>,
        _bold: bool,
    ) -> std::io::Result<()> {
        Ok(())
    }
    fn print_colored(
        &mut self,
        _text: &str,
        _color: Option<termcolor::Color>,
        _bold: bool,
    ) -> std::io::Result<()> {
        Ok(())
    }
    fn read_input(&self) -> String {
        // Default to "1" so interactive selection (difficulty/menu) proceeds in tests
        String::from("1")
    }

    fn read_char(&self) -> Option<char> {
        // Use the first character of the input if present
        self.read_input().chars().next()
    }
    fn read_pass(&self) -> String {
        String::new()
    }
    fn clear(&self) {}
    fn set_color(&mut self, _color: Option<termcolor::Color>) {}
    fn change_language(&mut self) {}
    fn get_language_data(&self) -> &LanguageData {
        static LD: once_cell::sync::OnceCell<LanguageData> = once_cell::sync::OnceCell::new();
        LD.get_or_init(|| LanguageData::load(Language::Global))
    }
}

pub struct MockPrinter {
    pub inputs: Mutex<VecDeque<String>>,
    pub outputs: Mutex<Vec<String>>,
}

impl MockPrinter {
    pub fn with_inputs(inputs: Vec<&str>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(MockPrinter {
            inputs: Mutex::new(inputs.into_iter().map(|s| s.to_string()).collect()),
            outputs: Mutex::new(Vec::new()),
        }))
    }

    #[allow(dead_code)]
    pub fn pop_output(&self) -> Option<String> {
        self.outputs.lock().unwrap().pop()
    }
}

impl GameUI for MockPrinter {
    fn print_message(
        &mut self,
        key: crate::messages::MessageKey,
        _color: Option<termcolor::Color>,
        extras: Option<&str>,
        _bold: bool,
    ) -> std::io::Result<()> {
        let mut out = self.outputs.lock().unwrap();
        if let Some(ex) = extras {
            out.push(format!("{}: {}", key.as_str(), ex));
        } else {
            out.push(key.as_str().to_string());
        }
        Ok(())
    }

    fn print_colored(
        &mut self,
        text: &str,
        _color: Option<termcolor::Color>,
        _bold: bool,
    ) -> std::io::Result<()> {
        self.outputs.lock().unwrap().push(text.to_string());
        Ok(())
    }

    fn read_input(&self) -> String {
        self.inputs.lock().unwrap().pop_front().unwrap_or_default()
    }

    fn read_char(&self) -> Option<char> {
        self.read_input().chars().next()
    }

    fn read_pass(&self) -> String {
        self.read_input()
    }

    fn clear(&self) {
        self.outputs.lock().unwrap().push("<clear>".to_string());
    }

    fn set_color(&mut self, _color: Option<termcolor::Color>) {}

    fn change_language(&mut self) {
        self.outputs
            .lock()
            .unwrap()
            .push("<language_changed>".to_string());
    }

    fn get_language_data(&self) -> &LanguageData {
        static LD: once_cell::sync::OnceCell<LanguageData> = once_cell::sync::OnceCell::new();
        LD.get_or_init(|| LanguageData::load(Language::Global))
    }
}

impl GameUI for Arc<Mutex<MockPrinter>> {
    fn print_message(
        &mut self,
        key: crate::messages::MessageKey,
        color: Option<termcolor::Color>,
        extras: Option<&str>,
        bold: bool,
    ) -> std::io::Result<()> {
        self.lock().unwrap().print_message(key, color, extras, bold)
    }

    fn print_colored(
        &mut self,
        text: &str,
        color: Option<termcolor::Color>,
        bold: bool,
    ) -> std::io::Result<()> {
        self.lock().unwrap().print_colored(text, color, bold)
    }

    fn read_input(&self) -> String {
        self.lock().unwrap().read_input()
    }

    fn read_char(&self) -> Option<char> {
        self.lock().unwrap().read_char()
    }

    fn read_pass(&self) -> String {
        self.lock().unwrap().read_pass()
    }

    fn clear(&self) {
        self.lock().unwrap().clear()
    }

    fn set_color(&mut self, color: Option<termcolor::Color>) {
        self.lock().unwrap().set_color(color)
    }

    fn change_language(&mut self) {
        self.lock().unwrap().change_language()
    }

    fn get_language_data(&self) -> &LanguageData {
        static LD: once_cell::sync::OnceCell<LanguageData> = once_cell::sync::OnceCell::new();
        LD.get_or_init(|| LanguageData::load(Language::Global))
    }
}
