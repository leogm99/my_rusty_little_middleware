use std::{
    collections::{HashMap},
    sync::{Arc, Condvar, Mutex}
};

use super::queue_holder::QueueHolder;

/// Message queues for clients
///
/// This structure *will* be concurrently accessed
pub struct MessageQueueMap {
    queue_map: HashMap<String, Arc<(Mutex<QueueHolder>, Mutex<bool>, Condvar)>>,
}

impl MessageQueueMap {
    pub fn new() -> Self {
        MessageQueueMap {
            queue_map: HashMap::new(),
        }
    }

    pub fn define_queue(&mut self, name: String) -> Result<(), ()> {
        if self.queue_map.contains_key(&name) {
            return Err(());
        }
        self.queue_map
            .insert(name, Arc::new((Mutex::new(QueueHolder::new()), Mutex::new(true), Condvar::new())));
        Ok(())
    }

    pub fn get_existing(&self, name: String) -> Option<Arc<(Mutex<QueueHolder>, Mutex<bool>, Condvar)>> {
        if self.queue_map.contains_key(&name) {
            return Some(self.queue_map.get(&name).unwrap().clone());
        }
        None
    }
}
