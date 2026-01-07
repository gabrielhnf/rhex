use ratatui::prelude::{Widget, Rect, Buffer};
use ratatui::widgets::{Block, Paragraph};

use crate::create_window;
use crate::ui::windows::{Window, WindowState};

create_window!(Generic {
    pressed: bool,
});

impl Generic {
    fn render_body(&self, area: Rect, buf: &mut Buffer){
        let block = Block::bordered().title("Generic Pane");
        Paragraph::new(if self.pressed { "Pressed!" } else {"This is an example pane."}).block(block).render(area, buf);
    }

    fn press(&mut self) {
        self.pressed = !self.pressed
    }

    fn handle_events(&mut self, shortcut: char){
        match shortcut {
            't' => self.press(),
            _ => {}
        }

    }
}
