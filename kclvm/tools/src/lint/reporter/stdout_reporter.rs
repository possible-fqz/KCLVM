use std::collections::HashMap;

use indexmap::{IndexMap, IndexSet};

use crate::lint::lint::KCLLinter::Linter;
use crate::lint::message::message::MSG;

use super::super::message::message::Message;
use super::base_reporter::{DisplayMsg, ReporterKind};

pub struct StdoutReporter {
    kind: ReporterKind,
}

impl StdoutReporter {
    pub fn new() -> Self {
        Self {
            kind: ReporterKind::Stdout,
        }
    }
}

impl DisplayMsg for StdoutReporter {
    fn print_msg(self: &StdoutReporter, lint: &Linter) {
        for m in &lint.msgs {
            println!("{}", m);
            println!();
        }
        println!("Chech total {} files:", &lint.file_list.len());
        for (key, val) in lint.msgs_map.iter() {
            let MSG = lint.MSGS.get(key);
            match MSG {
                Some(M) => {
                    println!("{} {}: {}", val, key, M.short_info);
                }
                None => {
                    println!("{} {}: {}", val, key, "");
                }
            }
        }
        println!("KCL Lint: {} problems", &lint.msgs.len())
    }
}
