use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stdout(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if key.code == KeyCode::Char('j') {
                // Next Lab Result
                app.next_lab();
            }

            if key.code == KeyCode::Char('k') {
                // Previous Lab Result
                app.previous_lab();
            }

            if key.code == KeyCode::Char('h') {
                // Previous Patient
                app.previous_patient();
            }

            if key.code == KeyCode::Char('l') {
                // Next Patient
                app.next_patient();
            }

            if key.code == KeyCode::Char('u') {
                // Scroll Up Result
                unimplemented!();
            }

            if key.code == KeyCode::Char('n') {
                // Scroll Down Result
                unimplemented!();
            }

            if key.code == KeyCode::Char('r') {
                // Reload Application Data
                unimplemented!();
            }

            if key.code == KeyCode::Char('q') {
                // Quit
                break;
            }
        }
    }
    Ok(())
}
