use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    // Install color_eyre for enhanced error reporting.
    color_eyre::install()?;

    // Initialize the terminal using ratatui. This will set up the terminal for drawing.
    let terminal = ratatui::init();

    // Run the main application logic, passing the initialized terminal. The result will be returned to the caller.
    let result = run(terminal);

    // Restore the terminal state before exiting, even if an error occurs.
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    Ok(())
}
