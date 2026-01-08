use crossterm::event::KeyCode;
use ratatui::{prelude::{Buffer, Rect, Widget}, style::Stylize, widgets::{Block, Borders, Clear}};

use crate::{create_window, ui::themes::default::DefaultTheme};

create_window!(Leader {
    visible: bool,
});

impl Leader {
    fn render_body(&self, area: Rect, buf: &mut Buffer){
        if self.visible {
            Clear.render(area, buf);
            Block::default().borders(Borders::TOP).fg(DefaultTheme::fg()).render(area, buf);
        }
    }

    fn handle_events(&mut self, event: KeyCode){
        match event {
            KeyCode::Esc => self.visible = !self.visible,
            _ => {},
        }
    }
}
