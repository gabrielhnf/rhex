use ratatui::{layout::Rect, style::{Color, Stylize}, widgets::{Block, Widget}};


pub trait Pane {
    fn is_active(&self) -> bool;
    fn toggle_active(&mut self);

    fn get_cursor(&self) -> &(u16, u16);

    fn update_area(&mut self, area: Rect);
    fn get_area(&self) -> Option<&Rect>;

    fn render_cursor(&self, buf: &mut ratatui::prelude::Buffer);

    fn move_up(&mut self);
    fn move_down(&mut self);

    fn move_left(&mut self);
    fn move_right(&mut self);
}

pub struct PaneState {
    cursor: (u16, u16),
    is_active: bool,

    area: Option<Rect>,
}

impl PaneState {
    pub fn new() -> Self {
        Self {
            cursor: (0, 0),
            is_active: false,

            area: None,
        }
    }
}

impl Pane for PaneState {

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
    }

    fn get_cursor(&self) -> &(u16, u16){
        &self.cursor
    }

    fn update_area(&mut self, area: Rect) {
        self.area = Some(area);
    }

    fn get_area(&self) -> Option<&Rect>{
        self.area.as_ref()
    }

    fn render_cursor(&self, buf: &mut ratatui::prelude::Buffer) {
        let cursor_pos = Rect {
            x: self.cursor.0 + self.get_area().unwrap().x, 
            y: self.cursor.1 + self.get_area().unwrap().y, 
            height: 1, width: 1};
        Block::new().bg(Color::Yellow).render(cursor_pos, buf)
    }

    fn move_left(&mut self){
        match self.get_area() {
            Some(area) => {
                if self.cursor.0 < area.width - 1 {
                    self.cursor.0 += 1;
                }

            },
            None => {},
        }
    }

    fn move_right(&mut self){
        match self.get_area() {
            Some(_) => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                }

            },
            None => {},
        }
    }

    fn move_up(&mut self){
        match self.get_area() {
            Some(_) => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                }

            },
            None => {},
        }
    }

    fn move_down(&mut self){
        match self.get_area() {
            Some(area) => {
                if self.cursor.1 < area.height - 1 {
                    self.cursor.1 += 1;
                }

            },
            None => {},
        }
    }
}
