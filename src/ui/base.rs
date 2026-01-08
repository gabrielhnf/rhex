use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::Widget};

use crate::ui::{components::{hex::Hex}, utils::register::WindowRegister};

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
        self.window_register.register(Hex::new(None));
        self.window_register.register(Hex::new(None));
        self.window_register.register(Hex::new(None));
        self.window_register.register(Hex::new(None));
        self.window_register.calculate();

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
                if ev.modifiers.contains(KeyModifiers::CONTROL) && ev.code == KeyCode::Char('q') {
                    self.exit = true;
                } //Should be if modifier is CONTROL, App handles events.

                match ev.code {
                    KeyCode::Tab => self.window_register.switch(),
                    key => {
                        self.window_register.get_active().handle_event(key);
                    },
                }
            },
            _ => {},
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        self.window_register.initialize(area);
        self.window_register.calculate();
        self.window_register.render(buf);
    }
}
