use crate::server::client_handler::ClientHandler;

use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

use super::{
    message_queue_monitor::MessageQueueMonitor,
    runnable::{Runnable, Startable},
};

const ERR_CREATING: &'static str = "An error ocurred while creating a server instance";

/// Basic TCP server
///
/// Launches a new thread for each new client
pub struct Server {
    listener: TcpListener,
    addr: String,
    should_exit: AtomicBool,
    // por que no podría poner un RefCell aca?
    // hint: refcell no es seguro entre hilos ;)
    // handlers: RefCell<Vec<JoinHandle<()>>>
    handlers: Mutex<Vec<JoinHandle<()>>>,
    queue_map: Arc<Mutex<MessageQueueMonitor>>,
}

impl Startable for Server {
    /// Accepts new TCP stream clients indefinately.
    ///
    /// The calling thread is locked until:
    /// - A message over a channel arrives to stop execution
    /// - The listener socket is closed
    /// - Any error is yielded by the `incoming` call to the listener
    fn start(&self) {
        for client in self.listener.incoming() {
            if self.should_exit.load(Ordering::SeqCst) {
                println!("Closing down");
                self.queue_map.lock().unwrap().print_queue();
                break;
            }
            match client {
                Ok(stream) => {
                    println!("New client connection");
                    let new_handler =
                        Arc::new(ClientHandler::new(stream, Arc::clone(&self.queue_map)));
                    if let Ok(join_handle) = ClientHandler::run(&new_handler) {
                        self.handlers.lock().unwrap().push(join_handle);
                    } else {
                        break;
                    }
                }
                Err(x) => {
                    println!("{}", x);
                    break;
                }
            }
        }
        let mut lock_handle = self.handlers.lock().unwrap();
        while let Some(handle) = lock_handle.pop() {
            handle.join().unwrap();
        }
    }
}

/// Server is now a runnable instance,
/// meaning it calls the `start` method within a thread
impl Runnable for Server {}

impl Server {
    /// Creates a new TCP listener server, ready to accept new connections.
    ///
    /// If `addr` is not bindable, we yield an error to the caller
    pub fn new(addr: &str) -> Result<Server, ()> {
        let listener = TcpListener::bind(addr);
        match listener {
            Ok(l) => Ok(Server {
                listener: l,
                addr: String::from(addr),
                should_exit: AtomicBool::new(false),
                handlers: Mutex::new(vec![]),
                queue_map: Arc::new(Mutex::new(MessageQueueMonitor::new())),
            }),
            Err(x) => {
                println!("{}: {}", ERR_CREATING, x);
                Err(())
            }
        }
    }

    /// Closes the listener socket, effectively shutting down any new connection
    ///
    /// The funny call to connect must be done
    /// in order to move forward the socket iterator
    /// (otherwise we should call libc's `close` on the raw file descriptor)
    pub fn close(&self) -> io::Result<()> {
        self.should_exit.store(true, Ordering::SeqCst);
        TcpStream::connect(&self.addr)?;
        Ok(())
    }
}
