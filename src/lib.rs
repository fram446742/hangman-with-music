use anyhow::Context;

pub mod console;
pub mod consts;
pub mod game;
pub mod hangman;
pub mod lang;
pub mod messages;
pub mod player;
pub mod printer;
pub mod tools;
pub mod ui;

/// Run the application in a loop. Returns an error if initialization fails.
pub fn run() -> anyhow::Result<()> {
    loop {
        let writer = crate::printer::ColorfulWriter::new(None);
        let boxed_printer: Box<dyn crate::ui::GameUI> = Box::new(writer);

        let mut gd = crate::game::GameData::new(boxed_printer, false)
            .context("Failed to initialize game data")?;
        gd.run()?;
        if !gd.ask_retry() {
            break;
        }
    }

    Ok(())
}
