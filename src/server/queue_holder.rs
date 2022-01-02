use std::{
    collections::VecDeque
};

pub struct QueueHolder {
    queue: VecDeque<String>
}

impl QueueHolder {
    pub fn new() -> Self {
        QueueHolder {
            queue: VecDeque::new()
        }
    }

    pub fn push(&mut self, message: String) {
        self.queue.push_back(message);
    }

    pub fn pop(&mut self) -> Option<(String, bool)> {
        let s = self.queue.pop_front();
        if s.is_some(){
            return Some((s.unwrap(), self.queue.is_empty()));
        }
        None
    }
}
