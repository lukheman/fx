use std::io;

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use ratatui::{
    layout::Alignment, layout::Rect, prelude::Stylize, style::Style, symbols::border, text::Text,
    widgets::Widget, widgets::*, Frame,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod tui;


#[derive(Default)]
struct App {
    entries: Vec<DirEntry>,
    exit: bool,
    area: Rect,
    path: PathBuf,
    state: ListState
}

impl Widget for &mut App {

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut styled_items = vec![];
        for item in &self.entries {
            if item.path().is_dir() {
                styled_items.push(
                    Text::from(item.file_name().into_string().unwrap()).style(Style::new().blue()),
                )
            } else {
                styled_items.push(
                    Text::from(item.file_name().into_string().unwrap()).style(Style::new().white()),
                )
            }
        }

        let block = Block::new()
            .title(self.path.to_str().unwrap())
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let items = List::new(styled_items.clone())
            .block(block)
            .highlight_symbol(">")
            .highlight_style(Style::new().black().on_blue());

        StatefulWidget::render(items, area, buf, &mut self.state);

    }
}

impl App {
     
    fn set_entries(&mut self) {
        self.entries.clear();

        let dir = fs::read_dir(self.path.clone()).unwrap();

        dir.for_each(|entry| {
            self.entries.push(entry.ok().unwrap());
        });

        self.reset_state();
    }

    fn draw(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.path = PathBuf::from("./");

        self.set_entries();

        while !self.exit {
            self.draw(terminal)?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.hande_key_event(key_event);
            }
            _ => {}
        }

        Ok(())
    }

    fn hande_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') => self.increment_selected_state(),
            KeyCode::Char('k') => self.decrement_selected_state(),
            KeyCode::Char('l') => self.next_path(),
            KeyCode::Char('h') => self.prev_path(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn increment_selected_state(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < (self.entries.len() - 1) {
                    i + 1
                } else {
                    0
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    fn decrement_selected_state(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    fn next_path(&mut self) {
        // check apakah path adalah dir
        let entry = &self.entries[self.state.selected().unwrap()];

        if entry.path().is_dir() {
            self.path = entry.path().to_path_buf();
            self.set_entries();
        }
    }

    fn prev_path(&mut self) {
        match self.path.parent() {
            Some(value) if value != Path::new("") => {
                self.path = value.to_path_buf();
                self.set_entries();
            }
            _ => {}
        }
    }

    fn reset_state(&mut self) {
        self.state.select(Some(0));
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    terminal.clear()?;

    let mut app = App::default();
    app.run(&mut terminal)?;

    tui::restore()?;

    Ok(())
}
