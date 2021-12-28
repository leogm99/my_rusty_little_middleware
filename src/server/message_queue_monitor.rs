use std::collections::{HashMap, VecDeque};

/// Message queues for clients
///
/// This structure *will* be concurrently accessed
pub struct MessageQueueMonitor {
    queue_map: HashMap<String, VecDeque<String>>,
}

impl MessageQueueMonitor {
    pub fn new() -> Self {
        MessageQueueMonitor {
            queue_map: HashMap::new(),
        }
    }

    pub fn define_queue(&mut self, name: String) -> Result<(), ()> {
        if self.queue_map.contains_key(&name) {
            return Err(());
        }
        self.queue_map.insert(name, VecDeque::new());
        Ok(())
    }

    pub fn push_to_queue(&mut self, name: String, message: String) -> Result<(), ()> {
        if !self.queue_map.contains_key(&name) {
            return Err(());
        }
        let message_queue = self.queue_map.get_mut(&name).unwrap();
        message_queue.push_back(message);
        Ok(())
    }

    pub fn pop_from_queue(&mut self, name: String) -> Result<String, ()> {
        if !self.queue_map.contains_key(&name) {
            return Err(());
        }

        let message_queue = self.queue_map.get_mut(&name).unwrap();
        if let Some(s) = message_queue.pop_front() {
            return Ok(s);
        }
        Err(())
    }

    pub fn print_queue(&self) {
        println!("{:?}", self.queue_map)
    }
}
