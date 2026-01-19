use rand::seq::IndexedRandom;
use serde_json::{Value, from_str};
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
    /// Load language data (messages and words) for the requested `language`.
    ///
    /// Falls back to an empty set of messages/words if the locale file cannot
    /// be parsed.
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
            "es" => include_str!("../assets/locales/es/messages.json"),
            "en" => include_str!("../assets/locales/en/messages.json"),
            "fr" => include_str!("../assets/locales/fr/messages.json"),
            "de" => include_str!("../assets/locales/de/messages.json"),
            "it" => include_str!("../assets/locales/it/messages.json"),
            "pt" => include_str!("../assets/locales/pt/messages.json"),
            "ru" => include_str!("../assets/locales/ru/messages.json"),
            "zh" => include_str!("../assets/locales/zh/messages.json"),
            "ja" => include_str!("../assets/locales/ja/messages.json"),
            _ => include_str!("../assets/locales/en/messages.json"),
        };

        let data: Value = match from_str(file_content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse language file for {}: {}", lang_code, e);
                Value::Object(serde_json::Map::new())
            }
        };

        let messages: HashMap<String, String> = data
            .as_object()
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let words: Vec<String> = data
            .get("Words")
            .and_then(|w| w.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Self { messages, words }
    }

    /// Return the localized message text for `key` or an error if the key is
    /// not present in the loaded locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangman::lang::{LanguageData, Language};
    /// let ld = LanguageData::load(Language::Global);
    /// let msg = ld.get_message("ContinueMessage").expect("message present");
    /// assert!(!msg.is_empty());
    /// ```
    pub fn get_message(&self, key: &str) -> Result<String, String> {
        self.messages
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Message with key '{}' not found", key))
    }

    /// Get a random word from the loaded word list, or `None` if there are no
    /// words available for the selected language.
    pub fn get_random_word(&self) -> Option<String> {
        let mut rng = rand::rng();
        self.words.choose(&mut rng).cloned()
    }
}
