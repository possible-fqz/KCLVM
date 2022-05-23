use std::collections::HashMap;
use std::hash::Hasher;

use indexmap::{IndexSet, IndexMap};

use super::super::message::message::{Message, MSG};
use super::stdout_reporter::StdoutReporter;
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ReporterKind {
    Stdout,
}

pub struct BaseReporter {
    pub kind: ReporterKind,
    pub sub_reporter: Box<dyn DisplayMsg>,
}

struct ReporterFacotry {}
impl ReporterFacotry {
    pub fn new_reporter(reporter: &ReporterKind) -> Box<dyn DisplayMsg> {
        match reporter {
            Stdout => Box::new(StdoutReporter::new()),
            _ => Box::new(StdoutReporter::new()),
        }
    }
}

impl BaseReporter {
    pub fn new(kind: ReporterKind) -> Self {
        let sub_reporter = ReporterFacotry::new_reporter(&kind);
        Self { kind, sub_reporter }
    }
    pub fn print_msg(&self, msgs: &IndexSet<Message>, msgs_map: &HashMap<String, u32>, MSGS: IndexMap<String, MSG>) {
        let c = &self.sub_reporter;
        c.print_msg(msgs, msgs_map, MSGS)
    }
}
pub trait DisplayMsg {
    fn print_msg(&self, msgs: &IndexSet<Message>, msgs_map: &HashMap<String, u32>, MSGS: IndexMap<String, MSG>);
}
