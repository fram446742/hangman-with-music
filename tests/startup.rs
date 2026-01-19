use hangman::game::GameData;
use hangman::ui::GameUI;
use tempfile::tempdir;
use termcolor::Color;
use hangman::lang::LanguageData;
use std::fs;

struct TestPrinter {
    lang: LanguageData,
}

impl TestPrinter {
    fn new() -> Self {
        TestPrinter { lang: LanguageData::load(hangman::lang::Language::Global) }
    }
}

impl GameUI for TestPrinter {
    fn print_message(&mut self, _key: hangman::messages::MessageKey, _color: Option<Color>, _extras: Option<&str>, _bold: bool) -> std::io::Result<()> {
        Ok(())
    }
    fn print_colored(&mut self, _text: &str, _color: Option<Color>, _bold: bool) -> std::io::Result<()> { Ok(()) }
    fn read_input(&self) -> String { String::from("1") }
    fn read_char(&self) -> Option<char> { None }
    fn read_pass(&self) -> String { String::new() }
    fn clear(&self) {}
    fn set_color(&mut self, _color: Option<Color>) {}
    fn change_language(&mut self) {}
    fn get_language_data(&self) -> &LanguageData { &self.lang }
}

#[test]
fn integration_headless_init_extracts_music() {
    let dir = tempdir().expect("tempdir");
    let printer = Box::new(TestPrinter::new());

    // Use the GameData constructor that allows specifying the music dir
    let _gd = GameData::new_with_music_dir(printer, false, dir.path().to_str().unwrap()).expect("game data new");

    // After initialization, the music directory should contain extracted music files
    let entries: Vec<_> = fs::read_dir(dir.path()).expect("read dir").filter_map(Result::ok).collect();
    let has_music = entries.iter().any(|e| {
        e.path().extension().map(|ext| ext.to_string_lossy().to_lowercase())
            .map(|s| ["mp3", "wav", "ogg", "flac", "aac"].contains(&s.as_str()))
            .unwrap_or(false)
    });

    assert!(has_music, "Expected extracted songs in the directory");
}
