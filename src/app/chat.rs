#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}

pub struct ChatState {
    pub path: Vec<usize>,
    pub messages: Vec<Message>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.path.clear();
        self.messages.clear();
    }
}
