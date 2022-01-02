use std::{io, net::TcpStream};

use super::command_dispatch::CommandDispatch;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Result<Client, ()> {
        match TcpStream::connect(addr) {
            Ok(stream) => Ok(Client { stream }),
            Err(x) => {
                eprintln!("{}", x);
                Err(())
            }
        }
    }

    pub fn loop_until_exit(&mut self) -> Result<(), ()> {
        let mut input = String::new();
        let stdin = &mut io::stdin();
        loop {
            input.clear();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    input.truncate(input.trim_end().len());
                    if input == "exit" {
                        break;
                    }
                    if let Err(_) = CommandDispatch::match_on_command(input.split(" ").collect(), &mut self.stream) {
                        return Err(());
                    }
                },
                Err(_) => break,
            }
        }
        Ok(())
    }
}
