use ratatui::style::Color;

pub struct DefaultTheme {
}

impl DefaultTheme {
    pub fn fg() -> Color {
        Color::Rgb(71, 237, 116)
    }
}
