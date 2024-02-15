use super::HISTORY;
use crate::println;

pub fn history(_arguments: &[&str]) {
    let history = HISTORY.lock();
    for command in history.iter() {
        println!("{}", command);
    }
}
