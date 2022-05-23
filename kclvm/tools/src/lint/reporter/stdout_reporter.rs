use std::collections::HashMap;

use indexmap::{IndexSet, IndexMap};

use crate::lint::message::message::MSG;

use super::super::message::message::{Message};
use super::base_reporter::{DisplayMsg, ReporterKind};

pub struct StdoutReporter {
    kind: ReporterKind
}

impl StdoutReporter {
    pub fn new() -> Self {
        Self {
            kind: ReporterKind::Stdout,
        }
    }
}

impl DisplayMsg for StdoutReporter {
    fn print_msg(self: &StdoutReporter, msgs: &IndexSet<Message>, msgs_map: &HashMap<String, u32>, MSGS: IndexMap<String, MSG>) {
        for m in msgs {
            println!("{}", m);
            println!();
        }
        for (key, val) in msgs_map.iter() {
            let MSG = MSGS.get(key);
            match MSG{
                Some(M) => {println!("{} {}: {}",val, key,  M.short_info);},
                None => {println!("{} {}: {}",val, key, "");},
            }
        }

        // for k, v in msgs_map:
        //     print("{:<8}{}: {}".format(v, k, self.linter.MSGS[k][1]))
        // print(f"KCL Lint: {len(self.linter.msgs)} problems")
    }
}
