use ratatui::widgets::{Block, Paragraph, Widget};

use crate::ui::pane::{Pane, PaneState};

pub struct GenericPane {
    pane_state: PaneState,
}

impl GenericPane {
    pub fn new() -> Self {
        Self { 
            pane_state: PaneState::new(),
        }
    }
}

impl Widget for &mut GenericPane {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        let block = Block::bordered().title("Generic Pane");
        let inner_area = block.inner(area);

        self.pane_state.update_area(inner_area);
        Paragraph::new("This is an example pane.").block(block).render(area, buf);

        self.pane_state.render_cursor(buf);
    }
}
