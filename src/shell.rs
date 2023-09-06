//! A basic shell with some simple commands

mod echo;

use alloc::string::ToString;

use crate::prelude::*;
use crate::shell::echo::echo;

type Program = Box<dyn Fn(&[String])>;

pub fn run_shell() {
	loop {
		let mut command = String::new();

		print!("$ ");

		read_line(&mut command);

		let arguments = command
			.split_whitespace()
			.map(ToString::to_string)
			.collect::<Vec<String>>();

		find_program(&arguments[0]).map_or_else(
			|| {
				println!("Unknown command: {}", arguments[0]);
			},
			|program| {
				program(&arguments[1..]);
			},
		);
	}
}

fn find_program(command_name: &str) -> Option<Program> {
	match command_name {
		"echo" => Some(Box::new(echo)),
		_ => None,
	}
}
