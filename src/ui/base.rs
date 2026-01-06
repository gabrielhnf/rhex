use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::{Widget}};

use crate::ui::{components::{generic::Generic, popup::OpenDialog}, utils::register::WindowRegister, windows::Window};

pub struct App {
    window_register: WindowRegister,
    file_prompt: OpenDialog,

    exit: bool,
}

//Implement some type of Event Register

impl App {
    pub fn new() -> Self {
        Self { 
            window_register: WindowRegister::new(),
            file_prompt: OpenDialog::new(String::new(), false),
            exit: false, 
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) {

        self.window_register.register(Generic::new());
        self.window_register.register(Generic::new());

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
                    //event::KeyCode::Tab => self.switch_window(),
                    event::KeyCode::Char(c) => self.file_prompt.append(c),
                    //self.event_handler.handle(active_window, char),
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        for window in &mut self.window_register {
            window.render(area, buf);
        }

        if self.file_prompt.is_visible() {
            let popup_area = Rect { x: 3*area.width/8, y: area.height/2 - 3, width: area.width/4, height: 3};
            self.file_prompt.render(popup_area, buf);
        }
    }
}
