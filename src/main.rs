fn main() -> anyhow::Result<()> {
    // Delegate to the library crate's run() function which handles the application loop
    hangman::run()?;
    Ok(())
}
