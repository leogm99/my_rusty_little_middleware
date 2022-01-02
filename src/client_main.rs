use std::{env, process};

use client::client::Client;

mod client;
mod common;

pub fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./client <IP> <PORT>");
        process::exit(1);
    }

    let addr = format!("{}:{}", args[1], args[2]);
    let mut client = Client::new(&addr)?;
    client.loop_until_exit()
}
