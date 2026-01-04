use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Widget};

use crate::ui::{pane::Pane, popup::popup::FilePrompt};

pub struct App {
    file_prompt: FilePrompt,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { 
            file_prompt: FilePrompt::new(),
            exit: false, 
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {
            match terminal.draw(|frame| self.draw(frame)) {
                Err(_) => {
                    todo!() //After X amounts of sequential frames dropped, quit.
                },
                _ => {}
            }

            //Handle events
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
            _ => {}, //Keep listening
        }
    }

    fn handle_event(&mut self) {
        match event::read() {
            Ok(Event::Key(ev)) => {
                match ev.code { //Events
                    event::KeyCode::Char('O') => self.file_prompt.toggle(),
                    event::KeyCode::Char('h') => self.file_prompt.pane.move_right(),
                    event::KeyCode::Char('j') => self.file_prompt.pane.move_down(),
                    event::KeyCode::Char('k') => self.file_prompt.pane.move_up(),
                    event::KeyCode::Char('l') => self.file_prompt.pane.move_left(),
                    event::KeyCode::Char('q') => self.exit = true,
                    event::KeyCode::Char(c) => self.file_prompt.append(c),
                    event::KeyCode::Backspace => self.file_prompt.pop(),
                    //self.event_handler.handle(active_window, char)
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        if self.file_prompt.is_visible() {
            let popup_area = Rect { x: 3*area.width/8, y: area.height/2 - 3, width: area.width/4, height: 3};
            self.file_prompt.render(popup_area, buf);
        }
    }
}
