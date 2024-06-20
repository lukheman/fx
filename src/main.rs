

use std::io;

mod tui;
mod app;

use ratatui::terminal::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io::stdout;

use app::App;

fn main() -> io::Result<()> {

    tui::init()?;

    let mut app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    run_app(&mut app, &mut terminal);

    tui::restore()?;
    Ok(())

}


fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {

        if let Event::Key(key) = event::read()? {

            match key.code {
                KeyCode::Char(0)
            }
        }

    }

    Ok(())
}
