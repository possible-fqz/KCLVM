use std::fmt::Display;
use std::process::Output;

use super::super::message::message::{Message, MSG};
use super::base_reporter::DisplayMsg;

pub struct StdoutReporter {}

impl StdoutReporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayMsg for StdoutReporter {
    fn print_msg(self: &StdoutReporter, msgs: &Vec<Message>) {
        for m in msgs {
            println!("{}", m);
        }
    }
}
