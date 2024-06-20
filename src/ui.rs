use crate::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut items = Vec::new();

    for entry in &app.items {
        let file_name = entry.file_name();

        if let Some(item) = file_name.to_str() {
            let i: ListItem;
            if entry.path().is_dir() {
                i = ListItem::new(item.to_string()).blue();
            } else {
                i = ListItem::new(item.to_string()).white();
            }

            items.push(i);
        }
    }

    let list = List::new(items)
        .block(list_block)
        .highlight_symbol("")
        .highlight_style(Style::default().black().on_blue());

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let preview_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let text = app.read_file();

    let preview_text = Paragraph::new(text).block(preview_block);

    f.render_widget(preview_text, chunks[1]);
}
