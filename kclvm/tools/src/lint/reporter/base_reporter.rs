use super::stdout_reporter::StdoutReporter;
use super::super::message::message::{Message, MSG};
#[derive(Debug)]
pub enum Reporter{
    STDOUT,
}

pub struct BaseReporter{
    pub kind: Reporter,
    pub sub_reporter: Box<dyn Display>,
}



struct ReporterFacotry{}
impl ReporterFacotry{
    pub fn new_reporter(reporter: &Reporter) -> Box<dyn Display>{
        match reporter{
            STDOUT => Box::new(StdoutReporter::new()),
            _ => Box::new(StdoutReporter::new()),
        }
    }
}

impl BaseReporter{
    pub fn new(kind: Reporter) -> Self{
        let sub_reporter = ReporterFacotry::new_reporter(&kind);
        Self { kind, sub_reporter }
    }
    pub fn print_msg(&self, msgs: &Vec<Message>) {
        let c = &self.sub_reporter;
        c.print_msg(msgs)
    }
}
pub trait Display {
    fn print_msg(&self, msgs: &Vec<Message>);

}

