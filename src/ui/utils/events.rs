pub struct EventHandler {
    current_context: usize,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            current_context: 0,
        }
    }

    pub fn context_switch(&mut self, context: usize){
        self.current_context = context;
    }
}
