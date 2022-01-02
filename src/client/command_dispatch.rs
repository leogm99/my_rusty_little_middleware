use std::{io::{ Write, Read, self}, net::TcpStream, str::FromStr};
use crate::common::command::*;

use super::command_serializer::CommandSerializer;

pub struct CommandDispatch {}

impl CommandDispatch {
	pub fn match_on_command(command: Vec<&str>, stream: &mut TcpStream) -> io::Result<()> {
		match command[0] {
			"define" => {
				if command.len() == 2 {
					CommandDispatch::send(&DefineCommand::new(command[1]), stream)?;
				}
			}
			"push" => {
				if command.len() == 3 {
					CommandDispatch::send(&PushCommand::new(command[2], command[1]), stream)?;
				}
			},
			"pop" => {
				if command.len() == 2 {
					CommandDispatch::send(&PopCommand::new(command[1]), stream)?;
				}
				match CommandDispatch::read_response(stream){
					Ok(s) => println!("{}", s),
					Err(_) => ()
				}
			}
			_ => ()
		}
		Ok(())
	}

	fn read_response(stream: &mut TcpStream) -> io::Result<String> {
		let mut buf: [u8; 2] = [0; 2];
		stream.read_exact(&mut buf)?;
		let len = u16::from_be_bytes(buf);
		let mut buf = Vec::with_capacity(len.into());
		buf.resize(len.into(), 0);
		stream.read_exact(&mut buf.as_mut_slice())?;
		let s = match std::str::from_utf8(&buf) {
			Ok(s) => Some(s),
			Err(_) => None
		};
		Ok(String::from_str(s.unwrap()).unwrap())
	}

	fn send(command: &dyn CommandSerializer, stream: &mut TcpStream) -> std::io::Result<()> {
		stream.write_all(command.serialize().as_slice())
	}
}
