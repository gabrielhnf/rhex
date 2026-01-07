use ratatui::{layout::Rect, style::{Color, Stylize}, widgets::{Block, Widget}};

pub trait Window {
    fn cursor(&self) -> &(u16, u16);
    fn cursor_mut(&mut self) -> &mut (u16, u16);

    fn area(&self) -> Option<&Rect>;
    fn set_area(&mut self, area: Rect);

    fn render_cursor(&self, buf: &mut ratatui::prelude::Buffer) {
        let cursor_pos = Rect {
            x: self.cursor().0 + self.area().unwrap().x, 
            y: self.cursor().1 + self.area().unwrap().y, 
            height: 1, width: 1};
        Block::new().bg(Color::Yellow).render(cursor_pos, buf)
    }

    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        self.set_render(area, buf);

        let inner_area = Rect {  //Area within the border
            x: area.x.saturating_add(1), 
            y: area.y.saturating_add(1), 
            width: area.width.saturating_sub(2), 
            height: area.height.saturating_sub(2) 
        };

        self.set_area(inner_area);
        self.render_cursor(buf);
    }

    fn set_render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer);
    fn handle_event(&mut self, shortcut: char);
}

pub trait Movable {
    fn cursor_mut(&mut self) -> &mut (u16, u16);
    fn cursor(&self) -> &(u16, u16);

    fn area(&self) -> &Option<Rect>;

    fn move_right(&mut self){
        match self.area() {
            Some(area) => {
                if self.cursor().0 < area.width - 1 {
                    self.cursor_mut().0 += 1;
                }
            },
            None => {},
        }
    }

    fn move_left(&mut self){
        match self.area() {
            Some(_) => {
                if self.cursor().0 > 0 {
                    self.cursor_mut().0 = self.cursor().0.saturating_sub(1);
                }
            },
            None => {},
        }
    }

    fn move_up(&mut self){
        match self.area() {
            Some(_) => {
                if self.cursor().1 > 0 {
                    self.cursor_mut().1 = self.cursor().1.saturating_sub(1);
                }
            },
            None => {},
        }
    }

    fn move_down(&mut self){
        match self.area() {
            Some(area) => {
                if self.cursor().1 < area.height - 1 {
                    self.cursor_mut().1 += 1;
                }
            },
            None => {},
        }
    }
}
