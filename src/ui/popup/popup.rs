use ratatui::widgets::{Block, Clear, Paragraph, Widget};

pub struct FilePrompt {
    input: String,
    visible: bool,
}

impl FilePrompt {
    pub fn new() -> Self {
        Self { 
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

    pub fn get_input(&self) -> &str { //Needed because String is owned by the struct and
                                      //Paragraph::new consumes String, but is behind a reference
                                      //to struct
        &self.input
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Widget for &FilePrompt {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        Clear.render(area, buf);
        Paragraph::new(self.get_input()).block(
            Block::bordered()
        ).render(area, buf);
    }
}
