use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, List, ListItem, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    // Install color_eyre for enhanced error reporting.
    let mut state = AppState::default();
    state.items.push(TodoItem {
        is_done: false,
        description: "Buy groceries".to_string(),
    });
    color_eyre::install()?;

    // Initialize the terminal using ratatui. This will set up the terminal for drawing.
    let terminal = ratatui::init();

    // Run the main application logic, passing the initialized terminal. The result will be returned to the caller.
    let result = run(terminal, &mut state);

    // Restore the terminal state before exiting, even if an error occurs.
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|frame| render(frame, app_state))?;
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

fn render(frame: &mut Frame, app_state: &AppState) {
    // app_state in this case is reborrowed by render function
    // as it has become an immutable reference from a mutable reference in the run function.
    // This allows us to read from app_state without modifying it.

    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .render(inner_area, frame.buffer_mut());
}
