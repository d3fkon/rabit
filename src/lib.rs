use crate::app::ui;
use app::{App, AppMode};
use crossterm::event::{self, Event, KeyCode};
use eyre::Result;
use std::{cell::RefCell, io::stdout, rc::Rc};

use tui::{backend::CrosstermBackend, Terminal};

pub mod app;

/// Start the UI of the application
pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Render Loop
    loop {
        let mut app = app.borrow_mut();
        terminal.draw(|rect| ui::draw(rect, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                app::AppMode::NORMAL => match key.code {
                    KeyCode::Char('q') => {
                        app.tracker.store_state();
                        break;
                    }
                    KeyCode::Char('k') => app.move_cursor_up(),
                    KeyCode::Char('j') => app.move_cursor_down(),
                    KeyCode::Char('h') => app.move_cursor_left(),
                    KeyCode::Char('l') => app.move_cursor_right(),
                    KeyCode::Char(' ') => app.mark_habit(),
                    KeyCode::Char(':') => app.enter_command_mode(),
                    _ => {}
                },
                app::AppMode::COMMAND => match key.code {
                    KeyCode::Esc => {
                        app.input = String::new();
                        app.mode = AppMode::NORMAL;
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Enter => {
                        app.execute_input();
                        app.mode = AppMode::NORMAL
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
