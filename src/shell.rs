//! A basic shell with some simple commands

mod clear;
mod echo;
mod history;

use alloc::borrow::ToOwned;

use spin::{Lazy, Mutex};

use crate::prelude::*;
use crate::shell::clear::clear;
use crate::shell::echo::echo;
use crate::shell::history::history;

type Program = Box<dyn Fn(&[&str])>;

/// Todo: eventually replace this with a history file.
static HISTORY: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Start running the systems shell.
///
/// Todo: when file system is implemented, make this into its own program.
pub fn run_shell() {
    loop {
        let mut command = String::new();

        print!("$ ");

        read_line(&mut command);

        let arguments = command.split_whitespace().collect::<Vec<&str>>();

        if arguments.is_empty() {
            continue;
        }

        find_program(arguments[0]).map_or_else(
            || {
                println!("Unknown command: {}", arguments[0]);
            },
            |program| {
                program(&arguments);
            },
        );

        add_to_history(&command);
    }
}

fn add_to_history(command: &str) { HISTORY.lock().push(command.to_owned()); }

fn find_program(command_name: &str) -> Option<Program> {
    match command_name {
        "echo" => Some(Box::new(echo)),
        "clear" => Some(Box::new(clear)),
        "history" => Some(Box::new(history)),
        _ => None,
    }
}
