
use std::path::PathBuf;
use std::fs::DirEntry;
use ratatui::widgets::ListState;
use std::fs;

pub struct App {
    pub path: PathBuf,
    pub items: Vec<DirEntry>,
    pub current_item: PathBuf,
    state: ListState
}

impl App {

    pub fn new() -> Self {

        Self {
            path: PathBuf::from("."),
            items: Vec::<DirEntry>::new(),
            current_item: PathBuf::default(),
            state: ListState::default(),
        }
    }

    fn read_path(&mut self) {
        let entriens = fs::read_dir(&self.path);

        for entry in entriens.unwrap() {
            self.items.push(entry.unwrap())
        }
    }

    fn replace_path(&mut self) {

        if self.current_item.is_dir() {
            self.path = self.current_item.clone();
            self.read_path();
        }

    }

    fn next_item(&mut self) {
        let i = match self.state.selected() {
            Some(value) => {
                if value >= self.items.len() - 1 {
                    0
                } else {
                    value + 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
    }

    fn prev_item(&mut self) {
        let i = match self.state.selected() {
            Some(value) => {
                if value == 0 {
                    self.items.len() - 1
                } else {
                    value - 1
                }
            }
            None => 0
        };

        self.state.select(Some(i));
    }

}
