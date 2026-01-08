use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use ratatui::layout::Rect;

use crate::create_window;
use crate::ui::windows::Window;

create_window!(OpenDialog{
    input: String,
    visible: bool,
});

impl OpenDialog {
    fn render_body(&self, area: Rect, buf: &mut ratatui::prelude::Buffer){
        Clear.render(area, buf);

        let block = Block::bordered();
        Paragraph::new(self.get_input()).block(block).render(area, buf);
    }

    pub fn handle_events(&mut self, event: KeyCode){}

    pub fn append(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
