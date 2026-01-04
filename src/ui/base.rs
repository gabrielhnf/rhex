use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Widget};

use crate::ui::popup::popup::FilePrompt;

pub struct App {
    active_pane: Panes,
    file_prompt: FilePrompt,
    exit: bool,
}

enum Panes {
    None,
    FilePrompt,
}

impl App {
    pub fn new() -> Self {
        Self { 
            active_pane: Panes::None,
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

    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }

    fn get_active_pane(&self) -> &Panes {
        &self.active_pane
    }

    fn set_active_pane(&mut self, pane: Panes) {
        self.active_pane = pane;
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
        let active_pane = self.get_active_pane();

        match event::read() {
            Ok(Event::Key(ev)) => {
                match ev.code { //Events
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('O') => {
                        self.file_prompt.toggle();
                        self.set_active_pane(
                            if self.file_prompt.is_visible() {
                                Panes::FilePrompt
                            } else {
                                Panes::None
                            });
                    },

                    KeyCode::Char(c) => {
                        match *active_pane {
                            Panes::FilePrompt => {
                                self.file_prompt.append(c);
                            }
                            Panes::None => {},
                        }
                    },
                    KeyCode::Backspace => {
                        match *active_pane {
                            Panes::FilePrompt => {
                                self.file_prompt.pop();
                            }
                            Panes::None => {},
                        }
                    }
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){
        if self.file_prompt.is_visible() {
            let popup_area = Rect { x: 3*area.width/8, y: area.height/2 - 3, width: area.width/4, height: 3};
            self.file_prompt.render(popup_area, buf);
        }
    }
}
