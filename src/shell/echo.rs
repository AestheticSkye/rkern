use crate::prelude::*;

pub fn echo(arguments: &[&str]) {
    let reply = arguments[1..].join(" ");
    println!("{}", reply);
}
