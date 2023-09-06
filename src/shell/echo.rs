use crate::prelude::*;

pub fn echo(arguments: &[String]) {
	let reply = arguments.join(" ");
	println!("{}", reply);
}
