use crate::ui::windows::Window;

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

    pub fn get_active(&mut self) -> &mut dyn Window {
        self.windows[self.active_window].as_mut()
    }
}

impl<'a> IntoIterator for &'a mut WindowRegister {
    type Item = &'a mut dyn Window;
    type IntoIter = std::iter::Map<std::slice::IterMut<'a, Box<dyn Window>>, fn(&'a mut Box<dyn Window>) -> &'a mut dyn Window>;

    fn into_iter(self) -> Self::IntoIter {
        self.windows.iter_mut().map(|b| b.as_mut())
    }
}
