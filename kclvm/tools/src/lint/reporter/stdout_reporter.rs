use super::base_reporter::Display;
use super::super::message::message::{Message, MSG};

pub struct StdoutReporter{}

impl StdoutReporter{
    pub fn new() -> Self{
        Self {  }
    }
}

impl Display for StdoutReporter {
    fn print_msg(self: &StdoutReporter, msgs: &Vec<Message>) {
        for m in msgs{
            print!("{:?}", m)
        }
    }
    
}