use crate::create_window;
use crate::ui::windows::Movable;
use crate::ui::themes::default::DefaultTheme;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, Widget};

create_window!(Hex {
    file_path: Option<String>,
});

impl Hex {
    fn render_body(&self, area: Rect, buf: &mut Buffer){
        let block = Block::bordered().border_set(border::ROUNDED).title(" Hex ".bold()).fg(DefaultTheme::fg());
        Paragraph::new(if self.file_path.is_some() { "" } else { "No file open."}).block(block).render(area, buf);
    }

    fn handle_events(&mut self, event: KeyCode){
        match event {
            KeyCode::Char(c) if c == 'h' => self.move_left(),
            KeyCode::Char(c) if c == 'j' => self.move_down(),
            KeyCode::Char(c) if c == 'k' => self.move_up(),
            KeyCode::Char(c) if c == 'l' => self.move_right(),
            _ => {}
        }
    }
}

impl Movable for Hex {
    fn cursor_mut(&mut self) -> &mut (u16, u16) {
        &mut self.cursor
    }

    fn cursor(&self) -> &(u16, u16) {
        &self.cursor
    }

    fn area(&self) -> &Option<Rect> {
        &self.area
    }
}
