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
use super::super::checker::base_checker::Check;
use crate::lint::checker::{self, imports::ImportCheck};
use std::collections::HashMap;
use super::config::Config;
// use std::collections::HashMap;
pub const LINT_CONFIG_SUFFIX: &str = ".kcllint";
pub const PARSE_FAILED_MSG_ID: &str = "E0999";


pub enum Checker {
    ImportCheck,
    MiscChecker,
    BasicChecker
}
struct CheckerFacotry{}
impl CheckerFacotry{
    pub fn new_checker(checker: &Checker, config: &'static Config) -> Box<dyn Check + 'static>{
        match checker{
            Checker::ImportCheck => Box::new(ImportCheck::new(&config)),
            _ => Box::new(ImportCheck::new(&config)),
        }
    }
}




pub struct Linter{
    pub config: Config,
    
}


#[test]
fn test_lint() {
    let lint: Linter = Linter { config: String::from("123") };
    let import_checker = CheckerFacotry::new_checker(
        &Checker::ImportCheck, &lint.config
    );
    let config = import_checker.check();
    println!("config: {}", config);
}
