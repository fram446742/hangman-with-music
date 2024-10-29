use rand::seq::SliceRandom;
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Language {
    Spanish,
    English,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
    Global,
}

#[derive(Debug, Clone)]
pub struct LanguageData {
    messages: HashMap<String, String>,
    words: Vec<String>,
}

impl LanguageData {
    pub fn load(language: Language) -> Self {
        let lang_code = match language {
            Language::Spanish => "es",
            Language::English => "en",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
            Language::Global => "en",
        };

        let file_content = match lang_code {
            "es" => include_str!("../src/locales/es/messages.json"),
            "en" => include_str!("../src/locales/en/messages.json"),
            "fr" => include_str!("../src/locales/fr/messages.json"),
            "de" => include_str!("../src/locales/de/messages.json"),
            "it" => include_str!("../src/locales/it/messages.json"),
            "pt" => include_str!("../src/locales/pt/messages.json"),
            "ru" => include_str!("../src/locales/ru/messages.json"),
            "zh" => include_str!("../src/locales/zh/messages.json"),
            "ja" => include_str!("../src/locales/ja/messages.json"),
            _ => include_str!("../src/locales/en/messages.json"),
        };

        let data: Value = from_str(file_content).expect("JSON was not well-formatted");

        let messages: HashMap<String, String> = data
            .as_object()
            .expect("Messages should be an object")
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
            .collect();

        let words: Vec<String> = data["Words"]
            .as_array()
            .expect("Words should be an array")
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

        Self { messages, words }
    }

    pub fn get_message(&self, key: &str) -> Result<String, String> {
        self.messages
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Message with key '{}' not found", key))
    }

    pub fn get_random_word(&self) -> Option<String> {
        self.words.choose(&mut rand::thread_rng()).cloned()
    }
}
