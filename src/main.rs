mod tui;
mod app;
mod ui;

use ratatui::terminal::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io::{self, stdout};

use app::App;

fn main() -> io::Result<()> {

    tui::init()?;

    let mut app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    app.read_path();
    run_app(&mut app, &mut terminal)?;

    tui::restore()?;
    Ok(())

}


fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {

            if key.kind == KeyEventKind::Press {

                match key.code {
                    KeyCode::Char('j') => app.next_item(),
                    KeyCode::Char('k') => app.prev_item(),
                    KeyCode::Char('l') => app.next_path(),
                    KeyCode::Char('h') => app.prev_path(),
                    KeyCode::Char('q') => break,
                    _ => {}
                }

            }
        }

    }

    Ok(())
}
