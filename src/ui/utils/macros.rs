#[macro_export]
macro_rules! create_window {
    (
        $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        pub struct $name {
            cursor: (u16, u16),
            area: Option<ratatui::layout::Rect>,
            $(pub $field: $ty),*
        }

        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self {
                    cursor: (0, 0),
                    area: None,
                    $($field),*
                }
            }
        }

        impl crate::ui::windows::Window for $name {
            fn cursor(&self) -> &(u16, u16){
                &self.cursor
            }

            fn cursor_mut(&mut self) -> &mut (u16, u16){
                &mut self.cursor
            }

            fn area(&self) -> Option<&ratatui::layout::Rect>{
                self.area.as_ref()
            }

            fn set_area(&mut self, area: ratatui::layout::Rect){
                self.area = Some(area)
            }

            fn set_render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
                self.render_body(area, buf);
            }

            fn handle_event(&mut self, event: crossterm::event::KeyCode){
                self.handle_events(event);
            }
        }

    };
}
