use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::Widget};

use crate::ui::{generic_pane::GenericPane, pane::Pane, popup::popup::FilePrompt};

pub struct App {
    pane_1: GenericPane,
    pane_2: GenericPane,

    file_prompt: FilePrompt,

    exit: bool,
}

//Implement some type of Event Register
//Implement some type of Window Register

impl App {
    pub fn new() -> Self {
        Self { 
            pane_1: GenericPane::new(),
            pane_2: GenericPane::new(),

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
                    //event::KeyCode::Tab => self.switch_window(),
                    event::KeyCode::Char('O') => self.file_prompt.toggle(),
                    event::KeyCode::Char('h') => self.file_prompt.pane.move_right(),
                    event::KeyCode::Char('j') => self.file_prompt.pane.move_down(),
                    event::KeyCode::Char('k') => self.file_prompt.pane.move_up(),
                    event::KeyCode::Char('l') => self.file_prompt.pane.move_left(),
                    event::KeyCode::Char('q') => self.exit = true,
                    event::KeyCode::Char(c) => self.file_prompt.append(c),
                    event::KeyCode::Backspace => self.file_prompt.pop(),
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
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50), 
                Constraint::Percentage(50)])
            .split(area);

        self.pane_1.render(split[0], buf);
        self.pane_2.render(split[1], buf);

        if self.file_prompt.is_visible() {
            let popup_area = Rect { x: 3*area.width/8, y: area.height/2 - 3, width: area.width/4, height: 3};
            self.file_prompt.render(popup_area, buf);
        }
    }
}
