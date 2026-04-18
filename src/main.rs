use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    widgets::{Paragraph, Widget},
};

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
    loop {
        // Rendering
        terminal.draw(render)?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            // Handle key events here. For example, you could check for specific keys to exit the loop.
            match key.code {
                // Exit the app
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame) {
    Paragraph::new("Hello, Ratatui!").render(frame.area(), frame.buffer_mut());
}
