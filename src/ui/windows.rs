use ratatui::{layout::Rect, style::{Color, Stylize}, widgets::{Block, Widget}};

pub trait Window {
    fn get_cursor(&self) -> &(u16, u16);
    fn set_cursor(&mut self, position: (u16, u16));

    fn get_area(&self) -> Option<&Rect>;
    fn set_area(&mut self, area: Rect);

    fn render_cursor(&self, buf: &mut ratatui::prelude::Buffer) {
        let cursor_pos = Rect {
            x: self.get_cursor().0 + self.get_area().unwrap().x, 
            y: self.get_cursor().1 + self.get_area().unwrap().y, 
            height: 1, width: 1};
        Block::new().bg(Color::Yellow).render(cursor_pos, buf)
    }

    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        self.set_render(area, buf);
        self.set_area(area);
        self.render_cursor(buf);
    }

    fn set_render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer);
}

pub struct WindowState {
    cursor: (u16, u16),
    area: Option<Rect>,
}

impl WindowState {
    pub fn new() -> Self {
        Self { cursor: (0, 0), area: None }
    }

    pub fn get_cursor(&self) -> &(u16, u16) {
        &self.cursor
    }

    pub fn set_cursor(&mut self, position: (u16, u16)) {
        self.cursor = position;
    }

    pub fn get_area(&self) -> Option<&Rect> {
        self.area.as_ref()
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }
}

#[macro_export]
macro_rules! create_window {
    (
        $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        pub struct $name {
            pub state: WindowState,
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self {
                    state: WindowState::new(),
                    $($field),*
                }
            }
        }

        impl Window for $name {
            fn get_cursor(&self) -> &(u16, u16) {
                &self.state.get_cursor()
            }

            fn set_cursor(&mut self, position: (u16, u16)) {
                self.state.set_cursor(position);
            }

            fn get_area(&self) -> Option<&Rect> {
                self.state.get_area()
            }

            fn set_area(&mut self, area: Rect) {
                self.state.set_area(area);
            }

            fn set_render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
                self.render_body(area, buf);
            }
        }
    };
}
