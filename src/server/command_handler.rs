use std::{
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::common::command::{DefineCommand, PopCommand, PushCommand};

const DEFINE: u8 = 0x64;
const PUSH: u8 = 0x75;
const POP: u8 = 0x6f;

use super::message_queue_monitor::MessageQueueMonitor;

pub trait ApplyCommand {
    fn apply_command(&mut self, queue_map: &Arc<Mutex<MessageQueueMonitor>>) -> Option<String>;
}

impl ApplyCommand for DefineCommand {
    fn apply_command(&mut self, queue_map: &Arc<Mutex<MessageQueueMonitor>>) -> Option<String> {
        let _ = queue_map
            .lock()
            .unwrap()
            .define_queue(self.queue_name_as_copy());
        None
    }
}

impl ApplyCommand for PushCommand {
    fn apply_command(&mut self, queue_map: &Arc<Mutex<MessageQueueMonitor>>) -> Option<String> {
        let _ = queue_map
            .lock()
            .unwrap()
            .push_to_queue(self.queue_name_as_copy(), self.message_as_copy());
        None
    }
}

impl ApplyCommand for PopCommand {
    fn apply_command(&mut self, queue_map: &Arc<Mutex<MessageQueueMonitor>>) -> Option<String> {
        match queue_map
            .lock()
            .unwrap()
            .pop_from_queue(self.queue_name_as_copy())
        {
            Ok(s) => Some(s),
            Err(()) => None,
        }
    }
}

pub struct Command;

impl Command {
    pub fn deserializer(
        command: u8,
        client_stream: &mut TcpStream,
    ) -> Option<Box<dyn ApplyCommand>> {
        let queue_name = Command::read_and_stringify_from_u16_len(client_stream);
        if queue_name.is_some() {
            return match command {
                DEFINE => Some(Box::new(DefineCommand::new(&queue_name.unwrap()))),
                PUSH => {
                    let message = Command::read_and_stringify_from_u16_len(client_stream);
                    if message.is_some() {
                        return Some(Box::new(PushCommand::new(
                            &queue_name.unwrap(),
                            &message.unwrap(),
                        )));
                    }
                    None
                }
                POP => Some(Box::new(PopCommand::new(&queue_name.unwrap()))),
                _ => None,
            };
        }
        None
    }

    fn read_and_stringify_from_u16_len(client_stream: &mut TcpStream) -> Option<String> {
        let mut len_buffer = [0; 2];
        if let Err(_) = client_stream.read_exact(&mut len_buffer) {
            return None;
        }
        let s_len = u16::from_be_bytes(len_buffer);
        let mut s_vec = Vec::with_capacity(s_len.into());
        s_vec.resize(s_len.into(), 0);
        if let Err(_) = client_stream.read_exact(&mut s_vec.as_mut_slice()) {
            return None;
        }
        if let Ok(s) = String::from_utf8(s_vec) {
            return Some(s);
        }
        None
    }
}
