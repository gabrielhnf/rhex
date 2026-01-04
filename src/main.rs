use ratatui::run;

use crate::ui::base::App;

mod ui;

fn main() {
    run(|terminal| App::new().run(terminal))
}
