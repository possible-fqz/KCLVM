use kclvm_ast::ast::{Program};
use super::super::message::message::{Message, MSG};
use super::super::lint::Linter::Linter;
use super::super::lint::config::Config;
pub struct BaseChecker<'c>{
    // checker name (you may reuse an existing one)
    pub name: String,
    // options level (0 will be displaying in --help, 1 in --long-help)
    // level = 1
    // messages constant to display
    pub MSGS: Vec<MSG>,
    // messages issued by this checker
    pub msgs: Vec<Message>,
    // mark this checker as enabled or not.
    // enabled: bool
    // The Linter which Checker belong to
    // pub lint: Linter,
    // ordered list of options to control the checker behaviour
    pub options: &'c Config, 
}

impl<'c> BaseChecker<'c>{
    pub fn new(name: String, MSGS: Vec<MSG>, msgs: Vec<Message>, options: &'c Config) -> Self{
        Self { name, MSGS, msgs, options }
    }
}

// pub trait Check {
//     fn check(&self, prog: Program);
// }
pub trait Check {
    fn check(&self) -> &Config;
}

