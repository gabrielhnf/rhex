use ratatui::widgets::{Block, Clear, Paragraph, Widget};

use crate::ui::pane::PaneState;
use crate::ui::pane::Pane;

pub struct FilePrompt {
    pub pane: PaneState,
    input: String,
    visible: bool,
}

impl FilePrompt {
    pub fn new() -> Self {
        Self { 
            pane: PaneState::new(),
            input: String::new(),
            visible: false,
        }
    }

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

impl Widget for &mut FilePrompt {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let block = Block::bordered();
        self.pane.update_area(block.inner(area));

        Clear.render(area, buf);
        self.pane.render_cursor(buf);
        Paragraph::new(self.get_input()).block(block).render(area, buf);
    }
}
