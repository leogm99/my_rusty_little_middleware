use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::{
    apply_command::Command,
    message_queue_map::MessageQueueMap,
    runnable::{Runnable, Startable},
};

pub struct ClientHandler {
    stream_socket: Mutex<TcpStream>,
    queue_map: Arc<Mutex<MessageQueueMap>>,
}

impl Startable for ClientHandler {
    fn start(&self) {
        let mut comm = [0; 1];
        let mut sock = self.stream_socket.lock().unwrap();
        loop {
            match sock.read_exact(&mut comm) {
                Ok(_) => {
                    if let Some(mut x) = Command::deserializer(comm[0], &mut sock) {
                        let s = x.apply_command(&self.queue_map);
                        if s.is_some() {
                            let response = s.unwrap().as_bytes().to_vec();
                            let size = &response.len().to_be_bytes()[6..8];
                            if let Err(_) = sock.write_all(&size) {
                                break;
                            }
                            if let Err(_) = sock.write_all(&response.as_slice()) {
                                break;
                            }
                        }
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
    pub fn new(stream: TcpStream, queue_map: Arc<Mutex<MessageQueueMap>>) -> Self {
        ClientHandler {
            stream_socket: Mutex::new(stream),
            queue_map,
        }
    }
}
