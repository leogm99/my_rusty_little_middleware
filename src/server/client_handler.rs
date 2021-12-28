use std::{
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::{
    command_handler::Command,
    message_queue_monitor::MessageQueueMonitor,
    runnable::{Runnable, Startable},
};

pub struct ClientHandler {
    stream_socket: Mutex<TcpStream>,
    queue_map: Arc<Mutex<MessageQueueMonitor>>,
}

impl Startable for ClientHandler {
    fn start(&self) {
        let mut comm = [0; 1];
        let mut sock = self.stream_socket.lock().unwrap();
        loop {
            match sock.read_exact(&mut comm) {
                Ok(_) => {
                    if let Some(mut x) = Command::deserializer(comm[0], &mut sock) {
                        let _ = x.apply_command(&self.queue_map);
                    }
                }
                Err(_) => {
                    break;
                }
            }
            comm[0] = 0;
        }
    }
}

impl Runnable for ClientHandler {}

impl ClientHandler {
    pub fn new(stream: TcpStream, queue_map: Arc<Mutex<MessageQueueMonitor>>) -> Self {
        ClientHandler {
            stream_socket: Mutex::new(stream),
            queue_map,
        }
    }
}
