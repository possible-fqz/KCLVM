use std::hash::{Hash, Hasher};

use super::super::lint::config::Config;
use super::super::lint::KCLLinter::Linter;
use super::super::message::message::{Message, MSG};
use super::imports::ImportChecker;
use super::misc::MiscChecker;
use indexmap::{IndexMap, IndexSet};
use kclvm_ast::ast::Program;
use kclvm_error::Diagnostic;
use kclvm_sema::resolver::scope::ProgramScope;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CheckerKind {
    ImportCheck,
    MiscChecker,
    BasicChecker,
}

pub struct BaseChecker {
    pub kind: CheckerKind,
    pub sub_checker: Box<dyn Check>,
    // options level (0 will be displaying in --help, 1 in --long-help)
    // level = 1
    // mark this checker as enabled or not.
    // enabled: bool
    // ordered list of options to control the checker behaviour
    // pub options: Config,
}

struct CheckerFacotry {}
impl CheckerFacotry {
    pub fn new_checker(checker: CheckerKind) -> Box<dyn Check> {
        match checker {
            CheckerKind::ImportCheck => Box::new(ImportChecker::new()),
            CheckerKind::MiscChecker => Box::new(MiscChecker::new()),
            _ => Box::new(ImportChecker::new()),
        }
    }
}

impl BaseChecker {
    pub fn new(kind: CheckerKind) -> Self {
        let sub_checker = CheckerFacotry::new_checker(kind.clone());
        Self { kind, sub_checker }
    }

    pub fn check(
        &mut self,
        ctx: &(
            String,
            Vec<String>,
            Program,
            ProgramScope,
            IndexSet<Diagnostic>,
        ),
        cfg: &Config,
    ) {
        let c = &mut self.sub_checker;
        c.check(ctx, cfg)
    }

    pub fn get_msgs(&self) -> IndexSet<Message> {
        let c = &self.sub_checker;
        let msgs = c.get_msgs();
        msgs
    }

    pub fn get_MSGS(&self) -> IndexMap<String, MSG> {
        let c = &self.sub_checker;
        let msgs = c.get_MSGS();
        msgs
    }

    pub fn get_kind(&self) -> CheckerKind {
        let c = &self.sub_checker;
        let kind = c.get_kind();
        kind
    }
}

pub trait Check {
    fn check(
        &mut self,
        ctx: &(
            String,
            Vec<String>,
            Program,
            ProgramScope,
            IndexSet<Diagnostic>,
        ),
        cfg: &Config,
    );
    fn get_msgs(&self) -> IndexSet<Message>;
    fn get_MSGS(&self) -> IndexMap<String, MSG>;
    fn get_kind(&self) -> CheckerKind;
}

// impl Clone for Box<dyn Check>{
//     fn clone(&self) -> Box<dyn Check> {
//         match self.get_kind() {
//             CheckerKind::ImportCheck => Box::new(ImportChecker::new()),
//             CheckerKind::MiscChecker => Box::new(MiscChecker::new()),
//             _ => Box::new(ImportChecker::new()),
//         }
//     }
// }

// impl Hash for Box<dyn Check> {
//     fn hash<H>(&self, state: &mut H) where H: Hasher {
//         self.get_kind().hash(state)
//     }
// }

// impl PartialEq for Box<dyn Check> {
//     fn eq(&self, other: &Box<dyn Check>) -> bool {
//         self.get_kind() == other.get_kind()
//     }
// }

// impl Eq for Box<dyn Check> {}
