// KCLLinter class controls all inspection processes of lint: loading config, checking and generating reports.

// The workflow of KCLLinter is as follows:
// 1. Load config.
// 2. Find all KCL files under the 'path' from CLI arguments, and parse them to ast.Program.
// 3. Register checker and reporter according to config
// 4. Distribute ast to each checker for checking, and generate Message，which represents the result of check.
// 5. Linter collects Messages from all checkers.
// 6. Distribute Message to each reporter as output
// ┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                   KCLLinter                                                                 │
// │                                                                                                             │
// │      ┌───────────┐                  ┌─────────────────────────────────────────────────────────────────┐     │
// │      │  KCL file │                  │                             Checker                             │     │
// │      └───────────┘                  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │     │
// │            ↓                        │  │  importChecker  │  │  schemaChecker  │  │       ...       │  │     │
// │      ┌───────────┐                  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │      │  ast.Prog │       →          │  │  │  Message  │  │  │  │  Message  │  │  │  │  Message  │  │  │     │
// │      └───────────┘                  │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │                                     │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │                                     │  │  │  Message  │  │  │  │  Message  │  │  │  │  Message  │  │  │     │
// │                                     │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │      ┌──────────────────────┐       │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │      │      Config          │       │  │  │    ...    │  │  │  │    ...    │  │  │  │    ...    │  │  │     │
// │      │                      │       │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │      │   1 config           │       │  └─────────────────┘  └─────────────────┘  └─────────────────┘  │     │
// │      │   2 .kcllint         │       └─────────────────────────────────────────────────────────────────┘     │
// │      │   3 default_config   │                                                                               │
// │      │                      │                                        ↓                                      │
// │      │                      │       msgs_map -> MessageID: count                                            │
// │      └──────────────────────┘       msgs ->    ┌────────────────────────────────────────────────────┐       │
// │                                                │  ┌───────────┐  ┌───────────┐  ┌───────────┐       │       │
// │                                                │  │  Message  │  │  Message  │  │  Message  │       │       │
// │                                                │  └───────────┘  └───────────┘  └───────────┘       │       │
// │                                                └────────────────────────────────────────────────────┘       │
// │                                                                                                             │
// │                                                                      ↓                                      │
// │                                     ┌─────────────────────────────────────────────────────────────────┐     │
// │                                     │                              Reporter                           │     │
// │                                     │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐     │     │
// │                                     │  │  stdout   │  │   sarif   │  │   file    │  │   ...     │     │     │
// │                                     │  └───────────┘  └───────────┘  └───────────┘  └───────────┘     │     │
// │                                     └─────────────────────────────────────────────────────────────────┘     │
// │                                                                                                             │
// │                                                                                                             │
// │                                                                                                             │
// └─────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

use kclvm_ast::ast::Program;
use super::super::checker::{base_checker::{Checker, Checker::{ImportCheck,MiscChecker}, BaseChecker}};
use crate::lint::{checker::{self, imports::{ImportChecker, IMPORT_MSGS}}, message::message::{Message,MSG}, reporter::base_reporter::BaseReporter};
use std::collections::HashMap;
use super::config::Config;
use walkdir::WalkDir;
use crate::lint::reporter::base_reporter::Reporter;
use kclvm_sema::resolver::{resolve_program, scope::ProgramScope};
// use kclvm_sema::scope::ProgramScope;
// use std::collections::HashMap;
use kclvm_parser::parse_program;
pub const LINT_CONFIG_SUFFIX: &str = ".kcllint";
pub const PARSE_FAILED_MSG_ID: &str = "E0999";
use once_cell::sync::Lazy;

pub const Linter_MSGS: Lazy<Vec<MSG>> = Lazy::new(|| {
    vec![
        MSG{ 
            id: String::from("E0999"), 
            short_info: String::from("Parse failed."), 
            long_info: String::from("Parse failed:{}."),
        },
    ]
});



pub struct Linter{
    path: Option<String>,
    file_list: Vec<String>,
    checkers:  Vec<BaseChecker>,
    reporters: Vec<BaseReporter>,
    config: Config,
    msgs: Vec<Message>,
    MSGS: Vec<MSG>,
    msgs_map: HashMap<String, u32>,
}

impl Linter{
    pub fn new() -> Self{
        Self { 
            path: None, 
            file_list: vec![], 
            checkers: vec![], 
            reporters: vec![], 
            config: Config::DEFAULT_CONFIG(), 
            msgs: vec![], 
            MSGS: Linter_MSGS.to_vec(), 
            msgs_map: HashMap::new() ,
        }

    }

    fn reset(&mut self){
        self.reporters = vec![];
        self.checkers = vec![];
        self.MSGS = Linter_MSGS.to_vec();
        self.msgs = vec![];
        self.msgs_map = HashMap::new();
    }

    fn register_checkers(&mut self, checkers: Vec<Checker>){
        for c in checkers{
            let checker = BaseChecker::new(c);
            print!("{:?}1111\n", &checker.kind);
            self.checkers.push(checker);
        }
    }

    fn register_reporters(&mut self, reporters: Vec<Reporter>){
        for r in reporters{
            let reporter = BaseReporter::new(r);
            self.reporters.push(reporter);
        }
    }

    fn get_scope(&self, file: &str) -> ProgramScope{
        let mut prog = parse_program(file);
        let scope = resolve_program(&mut prog);
        scope
    }

    pub fn run(&mut self, file: &str){
        let scope = self.get_scope(file);
        self.register_checkers(vec![ImportCheck, MiscChecker]);
        self.register_reporters(vec![Reporter::STDOUT]);
        for c in &mut self.checkers{
            c.check();
            let msgs = c.get_msgs();
            // collect lint error
            for m in msgs{
                self.msgs.push(m)
            }
        }
        for r in &self.reporters{
            r.print_msg(&self.msgs);
        }
    }

}

#[test]
fn test_lint() {
    let mut lint = Linter::new();
    let file = "/Users/zz/code/KCLVM-ant/hello.k";
    println!("{}", &file);
    lint.run(&file);
}
