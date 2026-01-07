use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout}, widgets::Widget};

use crate::ui::{components::generic::Generic, utils::register::WindowRegister};

pub struct App {
    window_register: WindowRegister,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { 
            window_register: WindowRegister::new(),
            exit: false, 
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) {
        self.window_register.register(Generic::new(false));
        self.window_register.register(Generic::new(false));

        while !self.exit {
            match terminal.draw(|frame| self.draw(frame)) {
                Err(_) => {
                    todo!() //After X amounts of sequential frames dropped, quit.
                },
                _ => {}
            }

            self.event_listener()
        }
    }

    fn draw(&mut self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }

    fn event_listener(&mut self) {
        match event::poll(Duration::from_millis(16)) {
            Ok(has_event) if has_event => {
                self.handle_event();
            },
            _ => {},
        }
    }

    fn handle_event(&mut self) {
        match event::read() {
            Ok(Event::Key(ev)) => {
                match ev.code {
                    KeyCode::Tab => self.window_register.switch(),
                    KeyCode::Char(c) if c == 'q' => self.exit = true,
                    KeyCode::Char(c) => {
                        self.window_register.get_active().handle_event(c);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){

        let split = Layout::default().direction(ratatui::layout::Direction::Horizontal).constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(area);

        let mut i = 0;

        for window in &mut self.window_register {
            window.render(split[i], buf);
            i += 1;
        }

    }
}
