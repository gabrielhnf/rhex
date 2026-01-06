use crate::ui::windows::Window;

//Windows should hold if they are active or not (this should be trait of Window)
//They should not be responsible for the actual context switch, this should be done by the App
//Every Window must have it's own EventRegister, where they handle their own events
//The App must implement a WindowRegister, which is responsible for registering new windows and
//switching context between these windows.
//As such, the WindowRegister must take priority over all Windows (in case of events, such as Tab
//being pressed to switch active windows)

pub struct WindowRegister {
    windows: Vec<Box<dyn Window>>,
    active_window: usize,
}

impl WindowRegister {
    pub fn new() -> Self {
        Self { 
        windows: vec![],
        active_window: 0,
        }
    }

    pub fn register<T: Window + 'static>(&mut self, window: T){
        self.windows.push(Box::new(window));
    }

    pub fn switch(&mut self){
        self.active_window = (self.active_window + 1) % self.windows.len();
    }

    pub fn get_active(&self) -> usize {
        self.active_window
    }
}

impl<'a> IntoIterator for &'a mut WindowRegister {
    type Item = &'a mut dyn Window;
    type IntoIter = std::iter::Map<std::slice::IterMut<'a, Box<dyn Window>>, fn(&'a mut Box<dyn Window>) -> &'a mut dyn Window>;

    fn into_iter(self) -> Self::IntoIter {
        self.windows.iter_mut().map(|b| b.as_mut())
    }
}
