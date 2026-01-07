use ratatui::prelude::{Widget, Rect, Buffer};
use ratatui::widgets::{Block, Paragraph};

use crate::create_window;
use crate::ui::windows::{Window, WindowState};

create_window!(Generic {
});

impl Generic {
    fn render_body(&self, area: Rect, buf: &mut Buffer){
        let block = Block::bordered().title("Generic Pane");
        Paragraph::new("This is an example pane.").block(block).render(area, buf);
    }
}
