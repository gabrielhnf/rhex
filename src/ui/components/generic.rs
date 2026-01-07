use ratatui::prelude::{Widget, Rect, Buffer};
use ratatui::widgets::{Block, Paragraph};

use crate::create_window;
use crate::ui::windows::{Window, Movable};

create_window!(Generic {
    pressed: bool,
});

impl Generic {
    fn render_body(&self, area: Rect, buf: &mut Buffer){
        let block = Block::bordered().title("Generic Pane");
        Paragraph::new(if self.pressed { "Pressed!" } else {"This is an example pane."}).block(block).render(area, buf);
    }

    fn handle_events(&mut self, shortcut: char){
        match shortcut {
            'h' => self.move_left(),
            'j' => self.move_down(),
            'k' => self.move_up(),
            'l' => self.move_right(),
            _ => {}
        }
    }
}

impl Movable for Generic {
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
