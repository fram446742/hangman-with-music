#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKey {
    WelcomeBanner,
    StartMessage,
    RetryPrompt,
    Instructions,
    ContinueMessage,
    SettingsMenu,
    InvalidOption,
    DifficultyMenu,
    LanguageMenu,
    InsertPassword,
    AccessGranted,
    EasterEgg1,
    EasterEgg2,
    AccessDenied,
    PlayersMenu,
    InvalidCharacter,
    LetterAlreadyUsed,
    AcceptedLetter,
    IncorrectLetter,
    WordDisplay,
    Lives,
    GuessedLetters,
    Congratulations,
    GameOver,
    // Music related
    MusicExtracted,
    PlayingTestSong,
    MusicQueue,
}

impl MessageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageKey::WelcomeBanner => "WelcomeBanner",
            MessageKey::StartMessage => "StartMessage",
            MessageKey::RetryPrompt => "RetryPrompt",
            MessageKey::Instructions => "Instructions",
            MessageKey::ContinueMessage => "ContinueMessage",
            MessageKey::SettingsMenu => "SettingsMenu",
            MessageKey::InvalidOption => "InvalidOption",
            MessageKey::DifficultyMenu => "DifficultyMenu",
            MessageKey::LanguageMenu => "LanguageMenu",
            MessageKey::InsertPassword => "InsertPassword",
            MessageKey::AccessGranted => "AccessGranted",
            MessageKey::EasterEgg1 => "EasterEgg1",
            MessageKey::EasterEgg2 => "EasterEgg2",
            MessageKey::AccessDenied => "AccessDenied",
            MessageKey::PlayersMenu => "PlayersMenu",
            MessageKey::InvalidCharacter => "InvalidCharacter",
            MessageKey::LetterAlreadyUsed => "LetterAlreadyUsed",
            MessageKey::AcceptedLetter => "AcceptedLetter",
            MessageKey::IncorrectLetter => "IncorrectLetter",
            MessageKey::WordDisplay => "WordDisplay",
            MessageKey::Lives => "Lives",
            MessageKey::GuessedLetters => "GuessedLetters",
            MessageKey::Congratulations => "Congratulations",
            MessageKey::GameOver => "GameOver",
            MessageKey::MusicExtracted => "MusicExtracted",
            MessageKey::PlayingTestSong => "PlayingTestSong",
            MessageKey::MusicQueue => "MusicQueue",
        }
    }

    /// Convenience method that returns the localized message text or a sensible
    /// fallback if the message is missing from the loaded `LanguageData`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hangman::lang::{LanguageData, Language};
    /// use hangman::messages::MessageKey;
    /// let ld = LanguageData::load(Language::Global);
    /// let msg = MessageKey::ContinueMessage.message(&ld);
    /// assert!(!msg.is_empty());
    /// ```
    pub fn message(&self, language_data: &crate::lang::LanguageData) -> String {
        language_data
            .get_message(self.as_str())
            .unwrap_or_else(|_| format!("**Missing message for key: {}**", self.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::{Language, LanguageData};

    #[test]
    fn message_matches_get_message_when_present() {
        let ld = LanguageData::load(Language::Global);
        let expected = ld
            .get_message(MessageKey::ContinueMessage.as_str())
            .expect("message present");
        let got = MessageKey::ContinueMessage.message(&ld);
        assert_eq!(got, expected);

        // ensure get_message returns Err for a non-existing key
        assert!(ld.get_message("__no_such_key__").is_err());
    }
}
