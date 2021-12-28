mod common;
mod server;

use server::runnable::Runnable;
use server::server::Server;

use std::io;
use std::sync::Arc;

fn wait_to_exit() {
    let mut quit = String::new();
    loop {
        match io::stdin().read_line(&mut quit) {
            Ok(_) => {
                if quit.trim().eq("q") {
                    break;
                }
            }
            Err(x) => {
                println!("{}", x);
                break;
            }
        }
        quit.clear();
    }
}

fn main() -> Result<(), ()> {
    let server = Arc::new(Server::new("localhost:8080")?);
    let server_join_handle = Server::run(&server)?;

    // false positive leak del buffer de stdin (atexit)
    wait_to_exit();

    match server.close() {
        Ok(()) => (),
        Err(x) => panic!("{}", x),
    }

    server_join_handle.join().expect("Can't join, panicking");

    Ok(())
}
