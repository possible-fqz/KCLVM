use super::super::lint::config::Config;
use super::super::lint::Linter::Linter;
use super::super::message::message::{Message, MSG};
use super::imports::ImportChecker;
use super::misc::MiscChecker;
use indexmap::IndexSet;
use kclvm_ast::ast::Program;
use kclvm_error::Diagnostic;
use kclvm_sema::resolver::scope::ProgramScope;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum Checker {
    ImportCheck,
    MiscChecker,
    BasicChecker,
}

pub struct BaseChecker {
    pub kind: Checker,
    pub sub_checker: Box<dyn Check>,
    // options level (0 will be displaying in --help, 1 in --long-help)
    // level = 1
    // messages constant to display
    // pub MSGS: Vec<MSG>,
    // messages issued by this checker
    // pub msgs: Vec<Message>,
    // mark this checker as enabled or not.
    // enabled: bool
    // The Linter which Checker belong to
    // pub lint: Linter,
    // ordered list of options to control the checker behaviour
    // pub options: Config,
}

struct CheckerFacotry {}
impl CheckerFacotry {
    pub fn new_checker(checker: Checker) -> Box<dyn Check> {
        match checker {
            Checker::ImportCheck => Box::new(ImportChecker::new()),
            Checker::MiscChecker => Box::new(MiscChecker::new()),
            _ => Box::new(ImportChecker::new()),
        }
    }
}

impl BaseChecker {
    pub fn new(kind: Checker) -> Self {
        let sub_checker = CheckerFacotry::new_checker(kind.clone());
        Self { kind, sub_checker }
    }
    pub fn check(&mut self, ctx: &(Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>)) {
        let c = &mut self.sub_checker;
        c.check(ctx)
    }
    pub fn get_msgs(&self) -> Vec<Message> {
        let c = &self.sub_checker;
        let msgs = c.get_msgs();
        msgs
    }

    pub fn get_kind(&self) -> Checker {
        let c = &self.sub_checker;
        let kind = c.get_kind();
        kind
    }
}

pub trait Check {
    fn check(&mut self, ctx: &(Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>));
    fn get_msgs(&self) -> Vec<Message>;
    fn get_kind(&self) -> Checker;
}
