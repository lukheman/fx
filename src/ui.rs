
use ratatui::Frame;
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::widgets::{Block, List, Borders};
use ratatui::style::Style;
use crate::App;

pub fn ui(f: &mut Frame, app: &mut App) {

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());


    let mut items = Vec::new();

    for entry in &app.items {
        let file_name = entry.file_name();

        if let Some(item) = file_name.to_str() {
            items.push(item.to_string());
        }

    }

    let list = List::new(items) 
        .block(list_block)
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

}
