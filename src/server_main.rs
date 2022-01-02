mod common;
mod server;

use server::runnable::Runnable;
use server::server::Server;

use std::sync::Arc;
use std::{env, io, process};

fn wait_to_exit() {
    let quit = &mut String::new();
    let stdin = &mut io::stdin();
    loop {
        quit.clear();
        match stdin.read_line(quit) {
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./server <PORT>");
        process::exit(1);
    }

    let port = &args[1];
    let addr = String::from("localhost:") + port;
    let server = Arc::new(Server::new(&addr)?);
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
